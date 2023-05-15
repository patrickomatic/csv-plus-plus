# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Options::FileOptions do
  let(:output_filename) { 'test.xlsx' }
  let(:sheet_name) { 'Test' }
  let(:options) { described_class.new(sheet_name, output_filename) }

  describe '#initialize' do
    it 'provides defaults for most options' do
      expect(options.output_filename.to_s).to(eq('test.xlsx'))
      expect(options.offset).to(eq([0, 0]))
      expect(options.create_if_not_exists).to(be(false))
      expect(options.verbose).to(be(false))
      expect(options.key_values).to(eq({}))
    end
  end

  describe '#verbose_summary' do
    let(:options) do
      build(
        :file_options,
        output_filename:,
        backup: true,
        create_if_not_exists: true,
        key_values: { foo: 'bar' },
        verbose: true
      )
    end

    subject { options.verbose_summary }

    it { is_expected.to(match(/Backup.*true/m)) }
    it { is_expected.to(match(/Create sheet if.*true/m)) }
    it { is_expected.to(match(/key-values.*foo.*bar/m)) }
    it { is_expected.to(match(/Verbose.*true/m)) }
    it { is_expected.to(match(/Output.*test\.xlsx/m)) }
  end
end
