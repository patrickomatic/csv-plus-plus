# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Modifier
    # Validates and coerces modifier user inputs as they are parsed.
    #
    # Previously this logic was handled in the parser's grammar, but with the introduction of variable binding the
    # grammar is no longer context free so we need the parser to be a little looser on what it accepts and validate it
    # here.  Having this layer is also nice because we can provide better error messages to the user for what went
    # wrong during the parse.
    # rubocop:disable Metrics/ClassLength
    class ModifierValidator
      extend ::T::Sig

      sig { returns(::CSVPlusPlus::Modifier::Modifier) }
      attr_reader :modifier

      sig { params(modifier: ::CSVPlusPlus::Modifier::Modifier).void }
      # @param modifier [Modifier::Modifier] The modifier to set the validated attributes on.
      def initialize(modifier)
        @modifier = modifier
      end

      sig { params(border_side: ::String).void }
      # Validates that +border_side+ is 'all', 'top', 'bottom', 'left' or 'right'.
      #
      # @param border_side [::String, Modifier::BorderSide] The unvalidated user input
      #
      # @return [Set<Modifier::BorderSide>]
      def border=(border_side)
        @modifier.border = ::T.cast(
          one_of(:border, border_side, ::CSVPlusPlus::Modifier::BorderSide),
          ::CSVPlusPlus::Modifier::BorderSide
        )
      end

      sig { params(border_color: ::String).void }
      # Validates that +border_color+ is a hex color.
      #
      # @param border_color [::String] The unvalidated user input
      def bordercolor=(border_color)
        @modifier.bordercolor = color_value(:bordercolor, border_color)
      end

      sig { params(border_style: ::String).void }
      # Validates that +borderstyle+ is 'dashed', 'dotted', 'double', 'solid', 'solid_medium' or 'solid_thick'.
      #
      # @param border_style [::String] The unvalidated user input
      def borderstyle=(border_style)
        @modifier.borderstyle = ::T.cast(
          one_of(:borderstyle, border_style, ::CSVPlusPlus::Modifier::BorderStyle),
          ::CSVPlusPlus::Modifier::BorderStyle
        )
      end

      sig { params(color: ::String).void }
      # Validates that +color+ is a hex color.
      #
      # @param color [::String] The unvalidated user input
      def color=(color)
        @modifier.color = color_value(:color, color)
      end

      sig { params(repetitions: ::String).void }
      # Validates that +repetitions+ is a positive integer.
      #
      # @param repetitions [String] The unvalidated user input
      def expand=(repetitions)
        @modifier.expand = ::CSVPlusPlus::Modifier::Expand.new(repetitions: positive_integer(:expand, repetitions))
      end

      sig { params(font_color: ::String).void }
      # Validates that +font_color+ is a hex color.
      #
      # @param font_color [::String] The unvalidated user input
      def fontcolor=(font_color)
        @modifier.fontcolor = color_value(:fontcolor, font_color)
      end

      sig { params(font_family: ::String).void }
      # Validates that +font_family+ is a string that looks like a valid font family.  There's only so much validation
      # we can do here
      #
      # @param font_family [::String] The unvalidated user input
      def fontfamily=(font_family)
        @modifier.fontfamily = matches_regexp(
          :fontfamily,
          unquote(font_family),
          /^[\w\s]+$/,
          'It is not a valid font family.'
        )
      end

      sig { params(font_size: ::String).void }
      # Validates that +font_size+ is a positive integer
      #
      # @param font_size [::String] The unvalidated user input
      def fontsize=(font_size)
        @modifier.fontsize = positive_integer(:fontsize, font_size)
      end

      sig { params(text_format: ::String).void }
      # Validates that +text_format+ is 'bold', 'italic', 'strikethrough' or 'underline'.
      #
      # @param text_format [::String] The unvalidated user input
      def format=(text_format)
        @modifier.format = ::T.cast(
          one_of(:format, text_format, ::CSVPlusPlus::Modifier::TextFormat),
          ::CSVPlusPlus::Modifier::TextFormat
        )
      end

      sig { void }
      # Sets the row or cell to be frozen
      def freeze!
        @modifier.freeze!
      end

      sig { void }
      # Sets an infinite +Expand+ on the +Modifier+.
      def infinite_expand!
        @modifier.infinite_expand!
      end

      sig { params(halign: ::String).void }
      # Validates that +halign+ is 'left', 'center' or 'right'.
      #
      # @param value [String] The unvalidated user input
      def halign=(halign)
        @modifier.halign = ::T.cast(
          one_of(:halign, halign, ::CSVPlusPlus::Modifier::HorizontalAlign),
          ::CSVPlusPlus::Modifier::HorizontalAlign
        )
      end

      sig { params(note: ::String).void }
      # Validates that +note+ is a quoted string.
      #
      # @param note [::String] The unvalidated user input
      def note=(note)
        @modifier.note = note
      end

      sig { params(number_format: ::String).void }
      # Validates that +number_format+ is 'currency', 'date', 'date_time', 'number', 'percent', 'text', 'time' or
      # 'scientific'.
      #
      # @param value [String] The unvalidated user input
      def numberformat=(number_format)
        @modifier.numberformat = ::T.cast(
          one_of(:numberformat, number_format, ::CSVPlusPlus::Modifier::NumberFormat),
          ::CSVPlusPlus::Modifier::NumberFormat
        )
      end

      sig { params(valign: ::String).void }
      # Validates that +valign+ is 'top', 'center' or 'bottom'.
      #
      # @param valign [String] The unvalidated user input
      def valign=(valign)
        @modifier.valign = ::T.cast(
          one_of(:valign, valign, ::CSVPlusPlus::Modifier::VerticalAlign),
          ::CSVPlusPlus::Modifier::VerticalAlign
        )
      end

      sig { params(rule: ::String).void }
      # Validates that the conditional validating rules are well-formed.
      #
      # Pretty much based off of the Google Sheets API spec here:
      # @see https://developers.google.com/sheets/api/samples/data#apply_data_validation_to_a_range
      #
      # @param rule [String] The validation rule to apply to this row or cell
      def validate=(rule)
        @modifier.validate = a_data_validation(:validate, rule)
      end

      sig { params(var: ::String).void }
      # Validates +var+ is a valid variable identifier.
      #
      # @param var [::String] The unvalidated user input
      def var=(var)
        # TODO: I need a shared definition of what a variable can be (I guess the :ID token)
        @modifier.var = matches_regexp(:var, var, /^\w+$/, 'It must be a sequence of letters, numbers and _.').to_sym
      end

      private

      sig { params(str: ::String).returns(::String) }
      # XXX centralize this unquoting logic :(((
      def unquote(str)
        # TODO: I'm pretty sure this isn't sufficient and we need to deal with the backslashes
        str.gsub(/^['\s]*|['\s]*$/, '')
      end

      sig { params(modifier: ::Symbol, value: ::String).returns(::CSVPlusPlus::Modifier::DataValidation) }
      def a_data_validation(modifier, value)
        data_validation = ::CSVPlusPlus::Modifier::DataValidation.new(value)
        return data_validation if data_validation.valid?

        raise_error(modifier, value, message: data_validation.invalid_reason)
      end

      sig { params(modifier: ::Symbol, value: ::String).returns(::CSVPlusPlus::Color) }
      def color_value(modifier, value)
        unless ::CSVPlusPlus::Color.valid_hex_string?(value)
          raise_error(modifier, value, message: 'It must be a 3 or 6 digit hex code.')
        end

        ::CSVPlusPlus::Color.new(value)
      end

      sig { params(modifier: ::Symbol, value: ::String, regexp: ::Regexp, message: ::String).returns(::String) }
      def matches_regexp(modifier, value, regexp, message)
        raise_error(modifier, value, message:) unless value =~ regexp
        value
      end

      sig { params(modifier: ::Symbol, value: ::String, choices: ::T.class_of(::T::Enum)).returns(::T::Enum) }
      def one_of(modifier, value, choices)
        choices.deserialize(value.downcase.gsub('_', ''))
      rescue ::KeyError
        raise_error(modifier, value, choices:)
      end

      sig { params(modifier: ::Symbol, value: ::String).returns(::Integer) }
      def positive_integer(modifier, value)
        Integer(value.to_s, 10).tap do |i|
          raise_error(modifier, value, message: 'It must be positive and greater than 0.') unless i.positive?
        end
      rescue ::ArgumentError
        raise_error(modifier, value, message: 'It must be a valid (whole) number.')
      end

      sig do
        type_parameters(:E)
          .params(
            modifier: ::Symbol,
            bad_input: ::String,
            choices: ::T.nilable(::T.all(::T.type_parameter(:E), ::T.class_of(::T::Enum))),
            message: ::T.nilable(::String)
          ).returns(::T.noreturn)
      end
      def raise_error(modifier, bad_input, choices: nil, message: nil)
        raise(::CSVPlusPlus::Error::ModifierValidationError.new(modifier, bad_input:, choices:, message:))
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end
