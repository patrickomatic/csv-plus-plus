# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to an Excel file
    class OpenDocument < ::CSVPlusPlus::Writer::Writer
      extend ::T::Sig
      include ::CSVPlusPlus::Writer::FileBackerUpper

      sig { params(options: ::CSVPlusPlus::Options::FileOptions, position: ::CSVPlusPlus::Runtime::Position).void }
      # @param options [Options::FileOptions]
      # @param position [Runtime::Position]
      def initialize(options, position)
        super(position)

        @options = options
      end

      sig { override.params(template: ::CSVPlusPlus::Template).void }
      # write a +template+ to an OpenDocument file
      def write(template)
        # TODO
      end

      sig { override.void }
      # write a backup of the google sheet
      def write_backup
        backup_file(@options)
      end
    end
  end
end
