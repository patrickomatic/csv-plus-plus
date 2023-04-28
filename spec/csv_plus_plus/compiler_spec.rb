# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Compiler do
  let(:input) { '' }
  let(:key_values) { {} }
  let(:source_code) { build(:source_code, input:) }
  let(:position) { build(:position, input:) }
  let(:runtime) { build(:runtime, source_code:, position:) }
  let(:options) { build(:options, key_values:) }
  let(:compiler) { build(:compiler, runtime:, options:) }

  describe '.with_compiler' do
    let(:filename) { 'foo.csvpp' }

    it 'yields a Compiler' do
      expect { |b| described_class.with_compiler(options:, runtime:, &b) }
        .to(yield_with_args(described_class))
    end

    context 'with Options.verbose = true' do
      let(:options) { build(:options, verbose: true) }

      it 'yields a Compiler with #benchmark set' do
        described_class.with_compiler(options:, runtime:) do |compiler|
          expect(compiler.benchmark).not_to(be_nil)
        end
      end
    end
  end

  describe '#outputting!' do
    it 'yields control' do
      expect { |b| compiler.outputting!(&b) }
        .to(yield_with_args(runtime.position))
    end
  end

  describe '#compile_template' do
    let(:template) { compiler.compile_template }
    let(:input) { "foo0,bar0,baz0\nfoo1,bar1,baz1\nfoo2,bar2,baz2\n" }

    it 'creates rows' do
      expect(template.rows.length).to(eq(3))
    end

    it 'sets row.index' do
      expect(template.rows[0].index).to(eq(0))
      expect(template.rows[1].index).to(eq(1))
      expect(template.rows[2].index).to(eq(2))
    end

    context 'with cell modifiers' do
      let(:input) { 'foo,[[halign=right/format=bold]]bar,baz' }

      it 'creates cells with the modifiers' do
        expect(template.rows[0].cells[1].modifier.halign).to(eq(::CSVPlusPlus::Modifier::HorizontalAlign::Right))
        expect(template.rows[0].cells[1].modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
      end
    end

    context 'with cell modifiers with multiple values' do
      let(:input) { 'foo,[[halign=right/format=bold/format=italic]]bar,baz' }

      it 'creates cells with the modifiers' do
        expect(template.rows[0].cells[1].modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
        expect(template.rows[0].cells[1].modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Italic))
      end
    end

    context 'with row modifiers' do
      let(:input) { '![[halign=center/format=bold]]foo,bar,baz' }

      it 'creates rows with the modifiers' do
        expect(template.rows[0].modifier.halign).to(eq(::CSVPlusPlus::Modifier::HorizontalAlign::Center))
        expect(template.rows[0].modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
      end
    end
  end
end
