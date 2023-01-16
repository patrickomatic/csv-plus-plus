# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    ##
    # A class that can output a +Template+ to CSV
    class CSV < ::CSVPlusPlus::Writer::BaseWriter
      # write a +template+ to CSV
      def write(template)
        # TODO
      end

      protected

      def load_requires
        require('csv')
      end
    end
  end
end
