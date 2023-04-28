# typed: false
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
    ::Tempfile.new(%w[foo csvpp]).tap do |f|
      f.write('foo,bar,baz')
    end
  end
  let(:output_filename) { ::Pathname.new(output_file.path) }

  after do
    output_file.unlink
  end

  describe '#write_backup' do
    subject { writer.write_backup(options) }

    after { subject.unlink }

    context 'when the first backup file is taken' do
      it 'creates the backup file' do
        expect(subject).to(be_exist)
      end
    end

    context 'when all the backup file options are taken' do
      # TODO
    end
  end
end
