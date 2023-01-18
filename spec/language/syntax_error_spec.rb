# frozen_string_literal: true

require 'syntax_error'

describe ::CSVPlusPlus::Language::SyntaxError do
  let(:filename) { 'foo.csvpp' }
  let(:line_number) { 1 }
  let(:row_index) { nil }
  let(:cell) { nil }
  let(:cell_index) { nil }
  let(:runtime) { build(:runtime, filename:, cell:, line_number:, row_index:, cell_index:) }

  describe '#to_s' do
    let(:syntax_error) { described_class.new('Invalid token', 'this$![ is bad input', runtime) }

    subject { syntax_error.to_trace }

    it { is_expected.to(eq('csv++ foo.csvpp:1 Invalid token: "this$![ is bad input"')) }
  end

  describe '#to_trace' do
    let(:syntax_error) { described_class.new('Invalid token', 'this$![ is bad input', runtime) }

    subject { syntax_error.to_trace }

    it { is_expected.to(eq('csv++ foo.csvpp:1 Invalid token: "this$![ is bad input"')) }

    context 'with a row_index' do
      let(:row_index) { 0 }

      it { is_expected.to(eq('csv++ foo.csvpp:1[0] Invalid token: "this$![ is bad input"')) }
    end

    context 'with a cell and row index' do
      let(:line_number) { 1 }
      let(:row_index) { 0 }
      let(:cell) { build(:cell, index: cell_index) }
      let(:cell_index) { 5 }

      it { is_expected.to(eq('csv++ foo.csvpp:1[0,5] Invalid token: "this$![ is bad input"')) }
    end
  end

  describe '#to_verbose_trace' do
    let(:cell) { nil }
    let(:syntax_error) do
      described_class.new('Invalid token', 'this$![ is bad input', runtime, wrapped_error: ::StandardError.new('foo'))
    end

    subject { syntax_error.to_verbose_trace }

    it { is_expected.to(eq('csv++ foo.csvpp:1 Invalid token: "this$![ is bad input"')) }
  end
end
