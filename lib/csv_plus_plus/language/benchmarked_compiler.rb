# frozen_string_literal: true

require 'benchmark'

module CSVPlusPlus
  module Language
    # Extend a +Compiler+ class and add benchmark timings
    # @attr_reader timings [Array<Benchmark::Tms>] +Benchmark+ timings that have been accumulated by each step of
    #   compilation
    # @attr_reader benchmark [Benchmark] A +Benchmark+ instance
    module BenchmarkedCompiler
      attr_reader :benchmark, :timings

      # Wrap a +Compiler+ with our instance methods that add benchmarks
      def self.with_benchmarks(compiler, &block)
        ::Benchmark.benchmark(::Benchmark::CAPTION, 25, ::Benchmark::FORMAT, '> Total') do |x|
          # compiler = new(options:, runtime:, benchmark: x)
          compiler.extend(self)
          compiler.benchmark = x

          block.call(compiler)

          [compiler.timings.reduce(:+)]
        end
      end

      # @param benchmark [Benchmark] A +Benchmark+ instance
      def benchmark=(benchmark)
        @benchmark = benchmark
        @timings = []
      end

      # Time the Compiler#outputting! stage
      def outputting!
        time_stage('Writing the spreadsheet') { super }
      end

      protected

      def parse_code_section!
        time_stage('Parsing code section') { super }
      end

      def parse_csv_section!
        time_stage('Parsing CSV section') { super }
      end

      def expanding
        time_stage('Expanding rows') { super }
      end

      def resolve_all_cells!(template)
        time_stage('Resolving each cell') { super(template) }
      end

      private

      def time_stage(stage, &block)
        ret = nil
        @timings << @benchmark.report(stage) { ret = block.call }
        ret
      end
    end
  end
end
