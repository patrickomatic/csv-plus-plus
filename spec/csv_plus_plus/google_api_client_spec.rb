# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::GoogleApiClient do
  before do
    allow(::Google::Auth).to(receive(:get_application_default).and_return({}))
  end

  let(:test_class) { ::Class.new.include(::CSVPlusPlus::GoogleApiClient).new }

  describe '.sheets_client' do
    subject { test_class.sheets_client }

    it { is_expected.to(be_a(::Google::Apis::SheetsV4::SheetsService)) }

    it 'has authorization set' do
      expect(subject.authorization).not_to(be_nil)
    end
  end

  describe '.drive_client' do
    subject { test_class.drive_client }

    it { is_expected.to(be_a(::Google::Apis::DriveV3::DriveService)) }

    it 'has authorization set' do
      expect(subject.authorization).not_to(be_nil)
    end
  end
end
