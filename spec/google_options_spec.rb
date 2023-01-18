# frozen_string_literal: true

require_relative '../lib/options'

describe ::CSVPlusPlus::GoogleOptions do
  let(:sheet_id) { '#id' }
  let(:google_options) { described_class.new(sheet_id) }

  describe '#to_s' do
    subject { google_options.to_s }

    it { is_expected.to(eq('GoogleOptions(sheet_id: #id)')) }
  end

  describe '#verbose_summary' do
    subject { google_options.verbose_summary }

    it { is_expected.to(match(/Sheet ID.*\#id/m)) }
  end
end
