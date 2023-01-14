# frozen_string_literal: true

require 'scope'
require 'syntax_error'

describe ::CSVPlusPlus::Language::Scope do
  let(:runtime) { build(:runtime) }
  let(:code_section) { build(:code_section) }
  let(:scope) { described_class.new(code_section) }

  describe '#resolve_static_variables' do
    let(:ast) do
      build(:fn_call, name: :multiply, a: build(:variable, id: :bar), b: build(:variable, id: :foo))
    end
    let(:variables) do
      {
        bar: build(:number_one),
        foo: build(:fn_call_add),
        dep: ast
      }
    end
    let(:code_section) { build(:code_section, variables:) }

    subject { scope.resolve_static_variables!(runtime) }

    it 'resolves the variables in dep' do
      expect(subject[:dep]).to(eq(build(:fn_call, name: :multiply, a: variables[:bar], b: variables[:foo])))
    end

    context 'with runtime variables' do
      let(:ast) do
        build(:fn_call, name: :multiply, a: build(:variable, id: :rownum), b: build(:variable, id: :foo))
      end

      it 'resolves just the static variables in dep' do
        expect(subject[:dep]).to(
          eq(
            build(
              :fn_call,
              name: :multiply,
              a: build(:variable, id: :rownum),
              b: variables[:foo]
            )
          )
        )
      end
    end

    context 'with undefined variables' do
      let(:variables) { { foo: build(:variable, id: :thisdoesnotexist) } }

      it 'should raise a SyntaxError' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Language::SyntaxError))
      end
    end
  end

  describe '#resolve_variable' do
    let(:ast) do
      build(:fn_call, name: :multiply, a: build(:variable, id: :rownum), b: build(:variable, id: :foo))
    end

    subject { scope.resolve_variable(ast, :rownum, build(:number_one)) }

    it {
      is_expected.to(eq(build(:fn_call, name: :multiply, a: build(:number_one), b: build(:variable, id: :foo))))
    }
  end

  describe '#copy_tree_with_replacement' do
    let(:ast) do
      build(
        :fn_call,
        a: build(:fn_call, a: build(:fn_call, a: build(:variable_foo), b: build(:number_one)), b: build(:number_two)),
        b: build(:variable_bar)
      )
    end

    subject { scope.copy_tree_with_replacement(ast, :fooz, :bar) }

    it { is_expected.to(eq(ast)) }

    it { is_expected.not_to(be(ast)) }
  end

  describe 'to_s' do
    subject { scope.to_s }

    it { is_expected.to(eq('Scope(code_section: CodeSection(functions: {}, variables: {}))')) }
  end
end
