# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Writer::Merger do
  let(:test_class) { ::Class.new.include(described_class).new }

  describe '#merge_cell_value' do
    let(:existing_value) { 'existing value' }
    let(:new_value) { 'new value' }
    let(:options) { build(:file_options) }

    subject { test_class.merge_cell_value(existing_value:, new_value:, options:) }

    context 'when with options.overwrite_values = true' do
      let(:options) { build(:file_options, overwrite_values: true) }

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

    context 'when with options.overwrite_values = false' do
      let(:options) { build(:file_options, overwrite_values: false) }

      it { is_expected.to(eq(existing_value)) }

      context 'when new_value is nil' do
        let(:existing_value) { nil }

        it { is_expected.to(eq(new_value)) }
      end

      context 'when options.verbose = true and the value is being overwritten' do
        let(:options) { build(:file_options, overwrite_values: false, verbose: true) }

        before { expect(test_class).to(receive(:warn).once.with(/Keeping old value/)) }

        it { is_expected.to(eq(existing_value)) }
      end
    end
  end
end
