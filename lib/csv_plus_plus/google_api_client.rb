# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  # A convenience wrapper around Google's REST API client
  module GoogleApiClient
    # Get a +::Google::Apis::SheetsV4::SheetsService+ instance connected to the sheets API
    #
    # @return [Google::Apis::SheetsV4::SheetsService]
    def self.sheets_client
      ::Google::Apis::SheetsV4::SheetsService.new.tap do |s|
        s.authorization = ::Google::Auth.get_application_default(['https://www.googleapis.com/auth/spreadsheets'].freeze)
      end
    end

    # Get a +::Google::Apis::DriveV3::DriveService+ instance connected to the drive API
    #
    # @return [Google::Apis::DriveV3::DriveService]
    def self.drive_client
      ::Google::Apis::DriveV3::DriveService.new.tap do |d|
        d.authorization = ::Google::Auth.get_application_default(['https://www.googleapis.com/auth/drive.file'].freeze)
      end
    end
  end
end
