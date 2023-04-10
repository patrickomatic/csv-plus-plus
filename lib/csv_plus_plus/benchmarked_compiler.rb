# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Extend a +Compiler+ class and add benchmark timings
  #
  # @attr_reader timings [Array<Benchmark::Tms>] +Benchmark+ timings that have been accumulated by each step of
  #   compilation
  # @attr_reader benchmark [Benchmark::Report] A +Benchmark+ instance
  class BenchmarkedCompiler < ::CSVPlusPlus::Compiler
    extend ::T::Sig

    sig { returns(::Benchmark::Report) }
    attr_reader :benchmark

    sig { returns(::T::Array[::Benchmark::Tms]) }
    attr_reader :timings

    sig do
      params(
        options: ::CSVPlusPlus::Options,
        runtime: ::CSVPlusPlus::Runtime::Runtime,
        block: ::T.proc.params(compiler: ::CSVPlusPlus::Compiler).void
      ).void
    end
    # Instantiate a +::Compiler+ that can benchmark (time) it's stages. For better or worse, the only way that they
    # Benchmark library exposes it's +::Benchmark::Report+ is via a block, so this code also has to wrap with one
    #
    # @param options [Options]
    # @param runtime [Runtime]
    def self.with_benchmarks(options:, runtime:, &block)
      ::Benchmark.benchmark(::Benchmark::CAPTION, 25, ::Benchmark::FORMAT, '> Total') do |x|
        # compiler.extend(self)
        compiler = new(benchmark: x, options:, runtime:)
        block.call(compiler)
        [compiler.timings.reduce(:+)]
      end
    end

    sig do
      params(
        benchmark: ::Benchmark::Report,
        options: ::CSVPlusPlus::Options,
        runtime: ::CSVPlusPlus::Runtime::Runtime
      ).void
    end
    # @param benchmark [::Benchmark::Report]
    def initialize(benchmark:, options:, runtime:)
      super(options:, runtime:)

      @benchmark = ::T.let(benchmark, ::Benchmark::Report)
      @timings = ::T.let([], ::T::Array[::Benchmark::Tms])
    end

    sig { override.params(block: ::T.proc.params(runtime: ::CSVPlusPlus::Runtime::Runtime).void).void }
    # Time the Compiler#outputting! stage
    # rubocop:disable Naming/BlockForwarding
    def outputting!(&block)
      time_stage('Writing the spreadsheet') { super(&block) }
    end
    # rubocop:enable Naming/BlockForwarding

    protected

    sig { override.void }
    def parse_code_section!
      time_stage('Parsing code section') { super }
    end

    sig { override.returns(::T::Array[::CSVPlusPlus::Row]) }
    def parse_csv_section!
      time_stage('Parsing CSV section') { super }
    end

    sig { override.params(block: ::T.proc.void).void }
    # rubocop:disable Naming/BlockForwarding
    def expanding!(&block)
      time_stage('Expanding rows') { super(&block) }
    end
    # rubocop:enable Naming/BlockForwarding

    sig { override.params(block: ::T.proc.void).void }
    # rubocop:disable Naming/BlockForwarding
    def bind_all_vars!(&block)
      time_stage('Binding [[var=]]') { super(&block) }
    end
    # rubocop:enable Naming/BlockForwarding

    sig do
      override
        .params(template: ::CSVPlusPlus::Template)
        .returns(::T::Array[::T::Array[::CSVPlusPlus::Entities::Entity]])
    end
    def resolve_all_cells!(template)
      time_stage('Resolving each cell') { super(template) }
    end

    private

    sig do
      type_parameters(:R).params(
        stage: ::String,
        block: ::T.proc.returns(::T.type_parameter(:R))
      ).returns(::T.nilable(::T.type_parameter(:R)))
    end
    def time_stage(stage, &block)
      ret = ::T.let(nil, ::T.nilable(::T.type_parameter(:R)))
      @timings << ::T.unsafe(@benchmark.report(stage) { ret = block.call })
      ret
    end
  end
end
