# frozen_string_literal: true

describe ::CSVPlusPlus::Language::Scope do
  let(:runtime) { build(:runtime) }
  let(:scope) { described_class.new(runtime:) }

  #   describe 'code_section=' do
  #     let(:ast) do
  #       build(:fn_call, name: :multiply, arguments: [build(:variable, id: :bar), build(:variable, id: :foo)])
  #     end
  #     let(:variables) do
  #       {
  #         bar: build(:number_one),
  #         foo: build(:fn_call_add),
  #         dep: ast
  #       }
  #     end
  #     let(:functions) { {} }
  #     let(:code_section) { build(:code_section, variables:, functions:) }
  #
  #     before { scope.code_section = code_section }
  #
  #     describe '@variables' do
  #       subject { code_section.variables }
  #
  #       it 'resolves the variables in dep' do
  #         expect(subject[:dep]).to(
  #           eq(
  #             build(
  #               :fn_call,
  #               name: :multiply,
  #               arguments: [variables[:bar], variables[:foo]]
  #             )
  #           )
  #         )
  #       end
  #
  #       context 'with runtime variables' do
  #         let(:ast) do
  #           build(:fn_call, name: :multiply, arguments: [build(:variable, id: :rownum), build(:variable, id: :foo)])
  #         end
  #
  #         it 'resolves just the static variables in dep' do
  #           expect(subject[:dep]).to(
  #             eq(
  #               build(
  #                 :fn_call,
  #                 name: :multiply,
  #                 arguments: [
  #                   build(:variable, id: :rownum),
  #                   variables[:foo]
  #                 ]
  #               )
  #             )
  #           )
  #         end
  #       end
  #
  #       context 'with undefined variables' do
  #         let(:bad_variables) { { foo: build(:variable, id: :thisdoesnotexist) } }
  #         let(:bad_code_section) { build(:code_section, variables: bad_variables) }
  #
  #         it 'should raise a SyntaxError' do
  #           expect { scope.code_section = bad_code_section }
  #             .to(raise_error(::CSVPlusPlus::Error::SyntaxError))
  #         end
  #       end
  #     end
  #
  #     describe '@functions' do
  #       subject { code_section.functions }
  #
  #       it 'resolves function dependencies' do
  #         # TODO: not implemented yet
  #       end
  #
  #       context 'with builtin functions' do
  #         # TODO
  #       end
  #     end
  #   end

  describe '#resolve_cell_value' do
    let(:scope) { build(:scope, functions:, variables: { foo: build(:number_one) }, runtime:) }
    let(:functions) { {} }
    let(:runtime) { build(:runtime, cell:) }

    let(:fn_call_celladjacent) { build(:fn_call, name: :celladjacent, arguments: [build(:cell_reference, ref: 'A')]) }

    subject { scope.resolve_cell_value }

    context 'with a nil cell.ast' do
      let(:cell) { build(:cell) }

      it 'should return early' do
        expect { subject }
          .not_to(raise_error)
      end
    end

    context 'with a variable reference' do
      let(:cell) { build(:cell, value: '=$$foo', ast: build(:variable, id: :foo)) }

      it 'returns a copy of the ast with the value inserted' do
        expect(subject).to(eq(build(:number_one)))
      end
    end

    context 'with an undefined variable' do
      let(:cell) { build(:cell, value: '=$$itdoesnotexist', ast: build(:variable, id: :itdoesnotexist)) }

      it 'should raise a SyntaxError' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Error::SyntaxError))
      end
    end

    context 'with a function reference' do
      let(:fn_body) { build(:fn_call, name: :add, arguments: [build(:variable, id: :a), build(:variable, id: :b)]) }
      let(:functions) { { foo: build(:fn, name: :foo, arguments: %i[a b], body: fn_body) } }

      let(:ast) { build(:fn_call, name: :foo, arguments: [build(:number_one), build(:number_two)]) }
      let(:cell) { build(:cell, value: '=$$foo', ast:) }

      it 'replaces the function and resolves the arguments' do
        expect(subject).to(eq(build(:fn_call, name: :add, arguments: [build(:number_one), build(:number_two)])))
      end
    end

    context 'with a builtin function reference (celladjacent)' do
      let(:ast) { fn_call_celladjacent }
      let(:cell) { build(:cell, value: '=CELLADJACENT(A)', ast:) }

      it 'replaces the function call with the builtin function' do
        expect(subject).to(eq(build(:cell_reference, ref: 'A1')))
      end
    end

    context 'with a defined function that references a builtin' do
      let(:functions) { { foo: build(:fn, name: :foo, arguments: %i[], body: fn_call_celladjacent) } }

      let(:ast) { build(:fn_call, name: :foo, arguments: []) }
      let(:cell) { build(:cell, value: '=FOO()', ast:) }

      it 'resolves all the way down' do
        expect(subject).to(eq(build(:cell_reference, ref: 'A1')))
      end
    end
  end

  describe '#to_s' do
    subject { scope.to_s }

    it do
      is_expected.to(match(/Scope\(functions: .+, runtime: .+, variables: .+\)/))
    end
  end
end
