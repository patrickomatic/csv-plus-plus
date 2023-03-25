# frozen_string_literal: true

describe ::CSVPlusPlus::BenchmarkedCompiler do
  let(:test_class) do
    ::Class.new do
      def run_one_step
        expanding!
      end

      def run_all_steps
        parse_code_section!
        parse_csv_section!
        expanding!
        bind_all_vars!
        resolve_all_cells!(nil)
        outputting!
      end

      def parse_code_section!; end
      def parse_csv_section!; end
      def expanding!; end
      def bind_all_vars!; end
      def resolve_all_cells!(_template); end
      def outputting!; end
    end
  end

  let(:benchmark) { instance_double('Benchmark::Report') }
  let(:compiler) { test_class.new }

  before { subject.extend(described_class) }

  describe '#with_benchmarks' do
    it 'attaches timers to each step' do
      described_class.with_benchmarks(compiler, &:run_all_steps)
      expect(compiler.benchmark).not_to(be_nil)
    end
  end

  describe '@timings' do
    subject { compiler }
    before { subject.benchmark = benchmark }

    it 'is empty before doing anything' do
      expect(subject.timings).to(be_empty)
    end

    context 'after running each step' do
      before { expect(benchmark).to(receive(:report)).exactly(1).times.with('Expanding rows') }

      before { subject.run_one_step }

      it 'records an entry in @timings' do
        expect(subject.timings.length).to(eq(1))
      end
    end

    context 'after running each step' do
      before { expect(benchmark).to(receive(:report)).exactly(6).times }

      before { subject.run_all_steps }

      it 'records an entry in @timings for each stage' do
        expect(subject.timings.length).to(eq(6))
      end
    end
  end
end
