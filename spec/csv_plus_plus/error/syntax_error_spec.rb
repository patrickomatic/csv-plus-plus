# frozen_string_literal: true

describe ::CSVPlusPlus::Error::SyntaxError do
  let(:row_index) { nil }
  let(:cell) { nil }
  let(:cell_index) { nil }
  let(:runtime) { build(:runtime, cell:, line_number: 1, row_index:, cell_index:) }
  let(:test_class) do
    ::Class.new(described_class) do
      def error_message
        'this is an error'
      end
    end
  end

  describe '#to_s' do
    let(:syntax_error) { test_class.new(runtime) }

    subject { syntax_error.to_s }

    it { is_expected.to(eq('foo.csvpp:1 this is an error')) }
  end

  describe '#to_trace' do
    let(:syntax_error) { test_class.new(runtime) }

    subject { syntax_error.to_trace }

    it { is_expected.to(eq('foo.csvpp:1 this is an error')) }

    context 'with a row_index' do
      let(:row_index) { 0 }

      it { is_expected.to(eq('foo.csvpp:1[0] this is an error')) }
    end

    context 'with a cell and row index' do
      let(:row_index) { 0 }
      let(:cell) { build(:cell, index: cell_index) }
      let(:cell_index) { 5 }

      it { is_expected.to(eq('foo.csvpp:1[0,5] this is an error')) }
    end
  end

  describe '#to_verbose_trace' do
    let(:syntax_error) { test_class.new(runtime, wrapped_error: ::StandardError.new('foo')) }

    subject { syntax_error.to_verbose_trace }

    it { is_expected.to(eq('foo.csvpp:1 this is an error')) }

    context 'without a #wrapped_error' do
      let(:syntax_error) { test_class.new(runtime) }

      it { is_expected.to(eq('foo.csvpp:1 this is an error')) }
    end
  end
end
