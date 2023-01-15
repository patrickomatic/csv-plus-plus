# frozen_string_literal: true

require_relative '../lib/options'

describe ::CSVPlusPlus::GoogleOptions do
  let(:sheet_id) { '#id' }
  let(:google_options) { described_class.new(sheet_id) }

  describe '#to_s' do
    subject { google_options.to_s }

    it { is_expected.to(eq('GoogleOptions(sheet_id: #id)')) }
  end
end

describe ::CSVPlusPlus::Options do
  let(:options) { described_class.new }

  describe '#initialize' do
    it 'provides defaults for most options' do
      expect(options.offset).to(eq([0, 0]))
      expect(options.create_if_not_exists).to(be(false))
      expect(options.verbose).to(be(false))
      expect(options.key_values).to(eq({}))
    end
  end

  describe '#to_s' do
    subject { options.to_s }

    it do
      is_expected.to(
        eq(
          'Options(create_if_not_exists: false, google: GoogleOptions(sheet_id: ), key_values: {}, ' \
          'offset: [0, 0], verbose: false)'
        )
      )
    end
  end
end
