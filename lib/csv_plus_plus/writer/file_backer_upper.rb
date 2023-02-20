# frozen_string_literal: true

require 'fileutils'
require 'pathname'

module CSVPlusPlus
  module Writer
    # A module that can be mixed into any Writer that needs to back up it's @output_filename (all of them except Google
    # Sheets)
    module FileBackerUpper
      # I don't want to include a bunch of second/millisecond stuff in the filename  unless we
      # really need to. so try a less specifically formatted filename then get more specific
      DESIRED_BACKUP_FORMATS = [%(%Y_%m_%d-%I_%M%p), %(%Y_%m_%d-%I_%M_%S%p), %(%Y_%m_%d-%I_%M_%S_%L%p)].freeze
      private_constant :DESIRED_BACKUP_FORMATS

      # Assuming the underlying spreadsheet is file-based, create a backup of it
      def write_backup
        return unless ::File.exist?(@options.output_filename)

        # TODO: also don't do anything if the current backups contents haven't changed (do a md5sum or something)

        attempt_backups.tap do |backed_up_to|
          warn("Backed up #{@options.output_filename} to #{backed_up_to}") if @options.verbose
        end
      end

      private

      def attempt_backups
        attempted =
          # rubocop:disable Lint/ConstantResolution
          DESIRED_BACKUP_FORMATS.map do |file_format|
            # rubocop:enable Lint/ConstantResolution
            filename = format_backup_filename(file_format)
            backed_up_to = backup(filename)

            next filename unless backed_up_to

            return backed_up_to
          end

        raise(::CSVPlusPlus::Error, "Unable to write backup file despite trying these: #{attempted.join(', ')}")
      end

      def backup(filename)
        return if ::File.exist?(filename)

        ::FileUtils.cp(@options.output_filename, filename)
        filename
      end

      def format_backup_filename(file_format)
        pn = ::Pathname.new(@options.output_filename)
        pn.sub_ext("-#{::Time.now.strftime(file_format)}" + pn.extname)
      end
    end
  end
end
