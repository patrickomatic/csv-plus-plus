# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Parser::CellValue do
  let(:runtime) { build(:runtime) }

  describe '#parse' do
    subject { described_class.new.parse(cell_value, runtime) }

    describe 'without a formula' do
      let(:cell_value) { 'just a value' }

      it { is_expected.to(be_nil) }
    end

    describe 'a function call' do
      let(:cell_value) { '=MULTIPLY(5, 5)' }

      it do
        is_expected.to(
          eq(
            build(
              :fn_call,
              name: 'MULTIPLY',
              arguments: [
                build(:number, n: 5),
                build(:number, n: 5)
              ]
            )
          )
        )
      end
    end

    describe 'a function call with 3 arguments' do
      let(:cell_value) { '=SUM(A1, A2, A3)' }

      it do
        is_expected.to(
          eq(
            build(
              :fn_call,
              name: 'SUM',
              arguments: [
                build(:cell_reference, ref: 'A1'),
                build(:cell_reference, ref: 'A2'),
                build(:cell_reference, ref: 'A3')
              ]
            )
          )
        )
      end
    end

    describe 'a function call in infix notation' do
      let(:cell_value) { '=3 - 2' }

      it do
        is_expected.to(
          eq(
            build(
              :fn_call,
              name: :-,
              arguments: [
                build(:number, n: 3),
                build(:number, n: 2)
              ],
              infix: true
            )
          )
        )
      end
    end

    describe 'infix function calls with parenthesis for precedence' do
      let(:cell_value) { '=1 * (3 - 2)' }

      it do
        is_expected.to(
          eq(
            build(
              :fn_call,
              name: :*,
              arguments: [
                build(:number, n: 1),
                build(:fn_call, name: :-, arguments: [build(:number, n: 3), build(:number, n: 2)], infix: true)
              ],
              infix: true
            )
          )
        )
      end
    end

    describe 'a variable' do
      let(:cell_value) { '=$$foo' }

      it { is_expected.to(eq(build(:variable, id: 'foo'))) }
    end

    describe 'a double quoted string' do
      let(:cell_value) { '="test"' }

      it { is_expected.to(eq(build(:string, s: 'test'))) }
    end

    describe 'a cell reference' do
      let(:cell_value) { '=A1' }

      it { is_expected.to(eq(build(:cell_reference, ref: 'A1'))) }
    end
  end
end
