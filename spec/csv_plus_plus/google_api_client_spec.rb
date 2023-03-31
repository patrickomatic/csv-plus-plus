# typed: false
# frozen_string_literal: true

require('google/apis/drive_v3')
require('google/apis/sheets_v4')
require('googleauth')

describe ::CSVPlusPlus::GoogleApiClient do
  describe '.sheets_client' do
    subject { described_class.sheets_client }

    it { is_expected.to(be_a(::Google::Apis::SheetsV4::SheetsService)) }

    it 'has authorization set' do
      expect(subject.authorization).not_to(be_nil)
    end
  end

  describe '.drive_client' do
    subject { described_class.drive_client }

    it { is_expected.to(be_a(::Google::Apis::DriveV3::DriveService)) }

    it 'has authorization set' do
      expect(subject.authorization).not_to(be_nil)
    end
  end
end
