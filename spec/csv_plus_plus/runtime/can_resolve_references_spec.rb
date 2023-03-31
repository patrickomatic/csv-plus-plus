# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::CanResolveReferences do
  let(:runtime) { build(:runtime) }

  describe '#resolve_cell_value' do
    let(:variables) { { foo: build(:number_one) } }
    let(:functions) { {} }
    let(:runtime) { build(:runtime, cell:, functions:, variables:) }

    let(:fn_call_celladjacent) { build(:fn_call, name: :celladjacent, arguments: [build(:cell_reference, ref: 'A')]) }

    subject { runtime.resolve_cell_value }

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
end
