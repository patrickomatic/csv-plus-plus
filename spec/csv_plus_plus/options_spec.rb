# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Options do
  let(:options) { described_class.new }

  describe '#initialize' do
    it 'provides defaults for most options' do
      expect(options.offset).to(eq([0, 0]))
      expect(options.create_if_not_exists).to(be(false))
      expect(options.verbose).to(be(false))
      expect(options.key_values).to(eq({}))
      expect(options.google).to(be_nil)
    end
  end

  describe '#google_sheet_id' do
    before { options.google_sheet_id = '#id' }

    it 'creates a GoogleOptions' do
      expect(options.google.sheet_id).to(eq('#id'))
    end
  end

  describe '#to_s' do
    subject { options.to_s }

    it do
      is_expected.to(
        eq(
          'Options(create_if_not_exists: false, google: , key_values: {}, offset: [0, 0], sheet_name: , verbose: false)'
        )
      )
    end
  end

  describe '#validate' do
    subject { options.validate }

    it 'returns a validation string' do
      expect(subject).to(be_a(::String))
    end

    context 'with a google.sheet_id' do
      before { options.google_sheet_id = '#id' }

      it { is_expected.to(be_nil) }
    end

    context 'with an output_filename' do
      before { options.output_filename = 'foo.xls' }

      it { is_expected.to(be_nil) }
    end
  end

  describe '#verbose_summary' do
    let(:options) do
      build(
        :options,
        backup: true,
        create_if_not_exists: true,
        key_values: { foo: 'bar' },
        output_filename: 'foo.xls',
        verbose: true
      )
    end

    subject { options.verbose_summary }

    it { is_expected.to(match(/Backup.*true/m)) }
    it { is_expected.to(match(/Create sheet if.*true/m)) }
    it { is_expected.to(match(/key-values.*foo.*bar/m)) }
    it { is_expected.to(match(/Verbose.*true/m)) }
  end
end
