# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Writer::Merger do
  let(:test_class) { ::Class.new.include(described_class).new }

  describe '#merge_cell_value' do
    let(:existing_value) { 'existing value' }
    let(:new_value) { 'new value' }
    let(:options) { build(:file_options) }

    subject { test_class.merge_cell_value(existing_value:, new_value:, options:) }

    it { is_expected.to(eq(new_value)) }

    context 'when new_value is nil' do
      let(:new_value) { nil }

      it { is_expected.to(eq(existing_value)) }
    end

    context 'when options.verbose = true and the value is being overwritten' do
      let(:options) { build(:file_options, verbose: true) }

      before { expect(test_class).to(receive(:warn).once.with(/Overwriting existing value:.*with.*/)) }

      it { is_expected.to(eq(new_value)) }
    end
  end
end
