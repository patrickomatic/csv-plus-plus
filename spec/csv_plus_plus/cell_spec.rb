# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Cell do
  let(:modifier) { build(:modifier) }
  let(:cell) { described_class.new(row_index: 0, index: 1, value:, modifier:) }
  let(:compiler) { build(:compiler) }

  describe '#value' do
    subject { cell.value }

    context 'with leading and trailing spaces' do
      let(:value) { '   test   ' }
      it { is_expected.to(eq('test')) }
    end

    context "when it's all spaces" do
      let(:value) { '     ' }
      it { is_expected.to(be_nil) }
    end

    context "when it's nil" do
      let(:value) { nil }
      it { is_expected.to(be_nil) }
    end
  end
end
