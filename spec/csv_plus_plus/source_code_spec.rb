# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::SourceCode do
  let(:filename) { 'test.csvpp' }
  let(:input) do
    <<~INPUT
      # this is a test
      foo := A1
      bar := 42

      ---
      foo,bar,baz
      one,two,three
    INPUT
  end
  let(:source_code) { build(:source_code, input:, filename:) }

  describe '#initialize' do
    it 'sets @filename' do
      expect(source_code.filename).to(eq('test.csvpp'))
    end

    context 'when filename is nil' do
      let(:filename) { nil }

      it 'defaults to "stdin"' do
        expect(source_code.filename).to(eq('stdin'))
      end
    end

    it 'sets @length_of_file' do
      expect(source_code.length_of_file).to(eq(7))
    end

    it 'sets @length_of_code_section' do
      expect(source_code.length_of_code_section).to(eq(5))
    end

    it 'sets @length_of_csv_section' do
      expect(source_code.length_of_csv_section).to(eq(2))
    end
  end

  describe '#in_code_section?' do
    subject { source_code.in_code_section?(line_number) }

    context 'when in the code section' do
      let(:line_number) { 1 }

      it { is_expected.to(eq(true)) }
    end

    context 'when not in the code section' do
      let(:line_number) { 6 }

      it { is_expected.to(eq(false)) }
    end
  end

  describe '#in_csv_section?' do
    subject { source_code.in_csv_section?(line_number) }

    context 'when in the CSV section' do
      let(:line_number) { 6 }

      it { is_expected.to(eq(true)) }
    end

    context 'when not in the CSV section' do
      let(:line_number) { 1 }

      it { is_expected.to(eq(false)) }
    end
  end
end
