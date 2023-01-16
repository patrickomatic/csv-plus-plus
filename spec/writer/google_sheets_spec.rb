# frozen_string_literal: true

require_relative '../../lib/writer/google_sheets'

describe ::CSVPlusPlus::Writer::GoogleSheets do
  describe '#write' do
    let(:options) { build(:options) }
    let(:writer) { described_class.new(options) }
    let(:template) { build(:template) }

    before { writer.write(template) }
  end
end
