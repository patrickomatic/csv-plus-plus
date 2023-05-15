# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # A module that can be mixed into any Writer that needs to back up it's @output_filename (all of them except Google
    # Sheets)
    module FileBackerUpper
      include ::Kernel
      extend ::T::Sig

      # I don't want to include a bunch of second/millisecond stuff in the filename  unless we
      # really need to. so try a less specifically formatted filename then get more specific
      DESIRED_BACKUP_FORMATS = ::T.let(
        [
          %(%Y_%m_%d-%I_%M%p),
          %(%Y_%m_%d-%I_%M_%S%p),
          %(%Y_%m_%d-%I_%M_%S_%L%p)
        ].freeze,
        ::T::Array[::String]
      )
      private_constant :DESIRED_BACKUP_FORMATS

      sig { params(options: ::CSVPlusPlus::Options::FileOptions).returns(::T.nilable(::Pathname)) }
      # Assuming the underlying spreadsheet is file-based, create a backup of it
      def backup_file(options)
        return unless ::File.exist?(options.output_filename)

        # TODO: also don't do anything if the current backups contents haven't changed (do a md5sum or something)

        attempt_backups(options).tap do |backed_up_to|
          puts("Backed up #{options.output_filename} to #{backed_up_to}") if options.verbose
        end
      end

      private

      sig { params(options: ::CSVPlusPlus::Options::FileOptions).returns(::Pathname) }
      # rubocop:disable Metrics/MethodLength
      def attempt_backups(options)
        attempted =
          # rubocop:disable Lint/ConstantResolution
          DESIRED_BACKUP_FORMATS.map do |file_format|
            # rubocop:enable Lint/ConstantResolution
            filename = format_backup_filename(file_format, options.output_filename)
            backed_up_to = backup(filename, options.output_filename)

            next filename unless backed_up_to

            return backed_up_to
          end

        raise(
          ::CSVPlusPlus::Error::WriterError,
          "Unable to write backup file despite trying these: #{attempted.join(', ')}"
        )
      end
      # rubocop:enable Metrics/MethodLength

      sig { params(filename: ::Pathname, output_filename: ::Pathname).returns(::T.nilable(::Pathname)) }
      def backup(filename, output_filename)
        return if ::File.exist?(filename)

        ::FileUtils.cp(output_filename, filename)
        filename
      end

      sig { params(file_format: ::String, output_filename: ::Pathname).returns(::Pathname) }
      def format_backup_filename(file_format, output_filename)
        output_filename.sub_ext("-#{::Time.now.strftime(file_format)}" + output_filename.extname)
      end
    end
  end
end
