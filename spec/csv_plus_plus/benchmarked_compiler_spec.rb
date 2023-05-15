# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::BenchmarkedCompiler do
  let(:benchmark) { instance_double('Benchmark::Report') }
  let(:options) { build(:file_options) }
  let(:runtime) { build(:runtime) }
  let(:compiler) { described_class.new(benchmark:, options:, runtime:) }

  describe '#with_benchmarks' do
    it 'attaches benchmark to the compiler' do
      expect(compiler.benchmark).not_to(be_nil)
    end
  end

  describe '@timings' do
    subject { compiler }

    it 'is empty before doing anything' do
      expect(subject.timings).to(be_empty)
    end

    context 'after running a step' do
      before { expect(benchmark).to(receive(:report)).exactly(1).times.with('Writing the spreadsheet') }

      before { subject.outputting! { |_runtime| true } }

      it 'records an entry in @timings' do
        expect(subject.timings.length).to(eq(1))
      end
    end
  end
end
