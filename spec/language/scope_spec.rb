# frozen_string_literal: true

require 'scope'
require 'syntax_error'

describe ::CSVPlusPlus::Language::Scope do
  let(:runtime) { build(:runtime) }
  let(:scope) { described_class.new(runtime:) }

  describe 'code_section=' do
    let(:complicated_ast) do
      build(:fn_call, name: :multiply, a: build(:variable, id: :bar), b: build(:variable, id: :foo))
    end
    let(:variables) do
      {
        bar: build(:number_one),
        foo: build(:fn_call_add),
        dep: complicated_ast
      }
    end
    let(:functions) { {} }
    let(:code_section) { build(:code_section, variables:, functions:) }

    before { scope.code_section = code_section }

    describe '@variables' do
      subject { code_section.variables }

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
        let(:bad_variables) { { foo: build(:variable, id: :thisdoesnotexist) } }
        let(:bad_code_section) { build(:code_section, variables: bad_variables) }

        it 'should raise a SyntaxError' do
          expect { scope.code_section = bad_code_section }
            .to(raise_error(::CSVPlusPlus::Language::SyntaxError))
        end
      end
    end

    describe '@functions' do
      subject { code_section.functions }
      # TODO
    end
  end

  describe 'to_s' do
    subject { scope.to_s }

    it do
      is_expected.to(eq('Scope(code_section: , runtime: Runtime(cell: , row_index: 0, cell_index: ))'))
    end
  end
end
