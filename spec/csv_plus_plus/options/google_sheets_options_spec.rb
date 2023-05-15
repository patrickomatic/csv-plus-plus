# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Options::GoogleSheetsOptions do
  let(:sheet_id) { '1234' }
  let(:sheet_name) { 'foo' }
  let(:options) { described_class.new(sheet_name, sheet_id) }

  describe '#initialize' do
    it 'provides defaults for most options' do
      expect(options.offset).to(eq([0, 0]))
      expect(options.create_if_not_exists).to(be(false))
      expect(options.verbose).to(be(false))
      expect(options.key_values).to(eq({}))
      expect(options.sheet_id).to(eq('1234'))
    end
  end

  describe '#verbose_summary' do
    let(:options) do
      build(
        :google_sheets_options,
        sheet_id: '1234',
        backup: true,
        create_if_not_exists: true,
        key_values: { foo: 'bar' },
        verbose: true
      )
    end

    subject { options.verbose_summary }

    it { is_expected.to(match(/Sheet ID.*1234/m)) }
    it { is_expected.to(match(/Backup.*true/m)) }
    it { is_expected.to(match(/Create sheet if.*true/m)) }
    it { is_expected.to(match(/key-values.*foo.*bar/m)) }
    it { is_expected.to(match(/Verbose.*true/m)) }
  end
end
