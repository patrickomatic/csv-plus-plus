# frozen_string_literal: true

module CSVPlusPlus
  # The logic for how a row can expand
  #
  # @attr_reader ends_at [Integer, nil] Once the row has been expanded, where it ends at.
  # @attr_reader repetitions [Integer, nil] How many times the row repeats/expands.
  # @attr_reader starts_at [Integer, nil] Once the row has been expanded, where it starts at.
  class Expand
    attr_reader :ends_at, :repetitions, :starts_at

    # @param repetitions [Integer, nil] How many times this expand repeats.  If it's +nil+ it will expand infinitely
    #   (for the rest of the worksheet.)
    # @param starts_at [Integer, nil] The final location where the +Expand+ will start.  It's important to note that
    #   this can't be derived until all rows are expanded, because each expand modifier will push down the ones below
    #   it.  So typically this param will not be passed in the initializer but instead set later.
    def initialize(repetitions: nil, starts_at: nil)
      @repetitions = repetitions

      self.starts_at = starts_at unless starts_at.nil?
    end

    # Has the row been expanded?
    #
    # @return [boolean]
    def expanded?
      !@starts_at.nil?
    end

    # Does this infinitely expand?
    #
    # @return [boolean]
    def infinite?
      repetitions.nil?
    end

    # Mark the start of the row once it's been expanded, as well as where it +ends_at+.  When expanding rows each one
    # adds rows to the worksheet and if there are multiple `expand` modifiers in the worksheet, we don't know the final
    # +row_index+ until we're in the phase of expanding all the rows out.
    def starts_at=(row_index)
      @starts_at = row_index
      @ends_at = row_index + @repetitions unless infinite?
    end
  end
end
