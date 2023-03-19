# frozen_string_literal: true

require_relative 'modifier'

module CSVPlusPlus
  # Validates and coerces modifier values as they are parsed.
  #
  # Previously this logic was handled in the parser's grammar, but with the introduction of variable binding, the
  # grammar is no longer context free so we need the parser to be a little looser on what it accepts and validate it
  # here.  Having this layer is also nice because we can provide better error messages to the user for what went
  # wrong during the parse.
  class ValidatedModifier < ::CSVPlusPlus::Modifier
    # Validates that +border+ is 'all', 'top', 'bottom', 'left' or 'right'.
    #
    # @param value [String] The unvalidated user input
    def border=(value)
      super(one_of(:border, value, %i[all top bottom left right]))
    end

    # Validates that +bordercolor+ is a hex color.
    #
    # @param value [String] The unvalidated user input
    def bordercolor=(value)
      super(color_value(:bordercolor, value))
    end

    # Validates that +borderstyle+ is 'dashed', 'dotted', 'double', 'solid', 'solid_medium' or 'solid_thick'.
    #
    # @param value [String] The unvalidated user input
    def borderstyle=(value)
      super(one_of(:borderstyle, value, %i[dashed dotted double solid solid_medium solid_thick]))
    end

    # Validates that +color+ is a hex color.
    #
    # @param value [String] The unvalidated user input
    def color=(value)
      super(color_value(:color, value))
    end

    # Validates that +expand+ is a positive integer.
    #
    # @param value [String] The unvalidated user input
    def expand=(value)
      super(::CSVPlusPlus::Expand.new(positive_integer(:expand, value)))
    end

    # Validates that +fontcolor+ is a hex color.
    #
    # @param value [String] The unvalidated user input
    def fontcolor=(value)
      super(color_value(:fontcolor, value))
    end

    # Validates that +fontcolor+ is a hex color.
    def fontfamily=(value)
      super(matches_regexp(:fontfamily, value, /^[\w\s]+$/, 'It is not a valid font family.'))
    end

    # Validates that +fontsize+ is a positive integer
    #
    # @param value [String] The unvalidated user input
    def fontsize=(value)
      super(positive_integer(:fontsize, value))
    end

    # Validates that +format+ is 'bold', 'italic', 'strikethrough' or 'underline'.
    #
    # @param value [String] The unvalidated user input
    def format=(value)
      super(one_of(:format, value, %i[bold italic strikethrough underline]))
    end

    # Validates that +halign+ is 'left', 'center' or 'right'.
    #
    # @param value [String] The unvalidated user input
    def halign=(value)
      super(one_of(:halign, value, %i[left center right]))
    end

    # Validates that +note+ is a quoted string.
    #
    # @param value [String] The unvalidated user input

    # Validates that +numberformat+ is 'currency', 'date', 'date_time', 'number', 'percent', 'text', 'time' or
    # 'scientific'.
    #
    # @param value [String] The unvalidated user input
    def numberformat=(value)
      super(one_of(:nubmerformat, value, %i[currency date date_time number percent text time scientific]))
    end

    # Validates that +valign+ is 'top', 'center' or 'bottom'.
    #
    # @param value [String] The unvalidated user input
    def valign=(value)
      super(one_of(:valign, value, %i[top center bottom]))
    end

    # XXX
    #
    # @param value [String] The unvalidated user input
    def validation=(value)
      # XXX: this is really complicated
      #   condition: 'blank'
      #            | 'boolean'
      #            | 'boolean'                 ':' condition_value
      #            | 'boolean'                 ':' condition_value | condition_value
      #            | 'custom_formula'          ':' condition_value
      #            | 'date_after'              ':' relative_date
      #            | 'date_before'             ':' relative_date
      #            | 'date_between'            ':' condition_value condition_value
      #            | 'date_eq'                 ':' condition_value
      #            | 'date_is_valid'
      #            | 'date_not_between'        ':' condition_value condition_value
      #            | 'date_not_eq'             ':' condition_values
      #            | 'date_on_or_after'        ':' condition_value | relative_date
      #            | 'date_on_or_before'       ':' condition_value | relative_date
      #            | 'not_blank'
      #            | 'number_between'          ':' condition_value condition_value
      #            | 'number_eq'               ':' condition_value
      #            | 'number_greater'          ':' condition_value
      #            | 'number_greater_than_eq'  ':' condition_value
      #            | 'number_less'             ':' condition_value
      #            | 'number_less_than_eq'     ':' condition_value
      #            | 'number_not_between'      ':' condition_value condition_value
      #            | 'number_not_eq'           ':' condition_value
      #            | 'one_of_list'             ':' condition_values
      #            | 'one_of_range'            ':' A1_NOTATION
      #            | 'text_contains'           ':' condition_value
      #            | 'text_ends_with'          ':' condition_value
      #            | 'text_eq'                 ':' condition_value
      #            | 'text_is_email'
      #            | 'text_is_url'
      #            | 'text_not_contains'       ':' condition_value
      #            | 'text_not_eq'             ':' condition_values
      #            | 'text_starts_with'        ':' condition_value
      #
      #   condition_values: condition_values condition_value | condition_value
      #   condition_value: STRING
      #
      #   relative_date: 'past_year' | 'past_month' | 'past_week' | 'yesterday' | 'today' | 'tomorrow'
    end

    # Validates +variable+ is a valid variable identifier.
    #
    # @param value [String] The unvalidated user input
    def var=(value)
      # TODO: I need a shared definition of what a variable can be (I guess the :ID token)
      super(matches_regexp(:var, value, /^\w+$/, 'It must be a sequence of letters, numbers and _.').to_sym)
    end

    private

    def positive_integer(modifier, value)
      Integer(value, 10).tap do |i|
        raise_error(modifier, value, message: 'It must be positive and greater than 0.') unless i.positive?
      end
    rescue ::ArgumentError
      raise_error(modifier, value, message: 'It must be a valid (whole) number.')
    end

    def color_value(modifier, value)
      unless ::CSVPlusPlus::Color.valid_hex_string?(value)
        raise_error(modifier, value, message: 'It must be a 3 or 6 digit hex code.')
      end

      ::CSVPlusPlus::Color.new(value)
    end

    def one_of(modifier, value, choices)
      value.downcase.to_sym.tap do |v|
        raise_error(modifier, value, choices:) unless choices.include?(v)
      end
    end

    def matches_regexp(modifier, value, regexp, message)
      raise_error(modifier, value, message:) unless value =~ regexp
      value
    end

    def raise_error(modifier, bad_input, choices: nil, message: nil)
      raise(::CSVPlusPlus::Error::ModifierValidationError.new(modifier, bad_input, choices:, message:))
    end
  end
end
