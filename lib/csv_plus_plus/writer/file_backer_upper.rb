# frozen_string_literal: true

require 'fileutils'
require 'pathname'

module CSVPlusPlus
  module Writer
    # A mixin that can
    module FileBackerUpper
      # we don't want to include a bunch of second/millisecond stuff in the filename  unless we
      # really need to. so try a less specifically formatted filename then get more specific
      DESIRED_BACKUP_FORMATS = [%(%Y_%m_%d-%I_%M%p), %(%Y_%m_%d-%I_%M_%S%p), %(%Y_%m_%d-%I_%M_%S_%L%p)].freeze
      private_constant :DESIRED_BACKUP_FORMATS

      # Assuming the underlying spreadsheet is file-based, create a backup of it
      # rubocop:disable Metrics/MethodLength
      def write_backup
        return unless ::File.exist?(@options.output_filename)

        attempted = []
        backed_up_to = nil

        # rubocop:disable Lint/ConstantResolution
        DESIRED_BACKUP_FORMATS.find do |file_format|
          # rubocop:enable Lint/ConstantResolution
          filename = format_backup_filename(file_format)
          attempted << filename
          backed_up_to = backup(filename)

          break if backed_up_to
        end

        unless backed_up_to
          raise(::CSVPlusPlus::Error, "Unable to write backup file despite trying these: #{attempted.join(', ')}")
        end

        warn("Backed up #{@options.output_filename} to #{backed_up_to}") if @options.verbose
      end
      # rubocop:enable Metrics/MethodLength

      private

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
