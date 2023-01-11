# frozen_string_literal: true

require 'cell_value.tab'

describe ::CSVPlusPlus::Language::CellValueParser do
  let(:ec) { build(:execution_context) }

  describe '#parse' do
    subject { described_class.new.parse(cell_value, ec) }

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

    describe 'a variable' do
      let(:cell_value) { '=$$foo' }

      it { is_expected.to(eq(build(:variable, id: 'foo'))) }
    end

    describe 'a double quoted string' do
      let(:cell_value) { '="test"' }

      it { is_expected.to(eq(build(:string, s: 'test'))) }
    end
  end
end
