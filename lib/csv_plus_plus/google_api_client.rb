# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # A convenience wrapper around Google's REST API client
  module GoogleApiClient
    extend ::T::Sig

    sig { returns(::Google::Apis::SheetsV4::SheetsService) }
    # Get a +Google::Apis::SheetsV4::SheetsService+ instance configured to connect to the sheets API
    #
    # @return [Google::Apis::SheetsV4::SheetsService]
    def self.sheets_client
      ::T.must(
        @sheets_client ||= ::T.let(
          ::Google::Apis::SheetsV4::SheetsService.new.tap do |s|
            s.authorization = ::Google::Auth.get_application_default(['https://www.googleapis.com/auth/spreadsheets'].freeze)
          end,
          ::T.nilable(::Google::Apis::SheetsV4::SheetsService)
        )
      )
    end

    sig { returns(::Google::Apis::DriveV3::DriveService) }
    # Get a +Google::Apis::DriveV3::DriveService+ instance connected to the drive API
    #
    # @return [Google::Apis::DriveV3::DriveService]
    def self.drive_client
      ::T.must(
        @drive_client ||= ::T.let(
          ::Google::Apis::DriveV3::DriveService.new.tap do |d|
            d.authorization = ::Google::Auth.get_application_default(['https://www.googleapis.com/auth/drive.file'].freeze)
          end,
          ::T.nilable(::Google::Apis::DriveV3::DriveService)
        )
      )
    end
  end
end
