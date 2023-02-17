# frozen_string_literal: true

class TestWriter
  include ::CSVPlusPlus::Writer::FileBackerUpper

  attr_reader :options

  def initialize(options)
    @options = options
  end
end

describe ::CSVPlusPlus::Writer::FileBackerUpper do
  let(:options) { build(:options, output_filename:) }
  let(:writer) { ::TestWriter.new(options) }
  let(:output_file) do
    ::Tempfile.new(%w[foo csvpp]) do |f|
      f.write('foo,bar,baz')
    end
  end
  let(:output_filename) { output_file.path }

  after do
    output_file.unlink
  end

  describe '#write_backup' do
    subject { options.write_backup }

    context 'when the first desired choice is taken' do
      # TODO
    end

    context 'when the second desired choice is taken' do
      # TODO
    end
    context 'when the third desired choice is taken' do
      # TODO
    end
  end
end
