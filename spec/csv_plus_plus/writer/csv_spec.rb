# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Writer::CSV do
  let(:output_filename) { 'foo.csv' }
  let(:options) { build(:options, output_filename:) }
  let(:runtime) { build(:runtime) }
  let(:writer) { described_class.new(options, runtime) }

  after { ::File.delete(output_filename) if ::File.exist?(output_filename) }

  describe '#write' do
    let(:template) { build(:template) }

    subject { writer.write(template) }

    it 'writes to output_filename' do
      subject
      expect(::File).to(exist(output_filename))
    end
  end
end
