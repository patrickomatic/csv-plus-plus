# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to an Excel file
    class OpenDocument < ::CSVPlusPlus::Writer::BaseWriter
      extend ::T::Sig

      include ::CSVPlusPlus::Writer::FileBackerUpper

      sig { override.params(template: ::CSVPlusPlus::Template).void }
      # write a +template+ to an OpenDocument file
      def write(template)
        # TODO
      end
    end
  end
end
