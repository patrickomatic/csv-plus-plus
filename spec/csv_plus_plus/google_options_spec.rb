# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::GoogleOptions do
  let(:sheet_id) { '#id' }
  let(:google_options) { described_class.new(sheet_id) }

  describe '#verbose_summary' do
    subject { google_options.verbose_summary }

    it { is_expected.to(match(/Sheet ID.*\#id/m)) }
  end
end
