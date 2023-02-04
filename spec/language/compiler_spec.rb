# frozen_string_literal: true

require 'compiler'

describe ::CSVPlusPlus::Language::Compiler do
  let(:input) { '' }
  let(:key_values) { {} }
  let(:scope) { build(:scope, runtime:) }
  let(:runtime) { build(:runtime, input:) }
  let(:options) { build(:options, key_values:) }
  let(:compiler) { build(:compiler, runtime:, options:, scope:) }

  describe '.with_compiler' do
    let(:filename) { 'foo.csvpp' }

    it 'yields a Compiler' do
      expect { |b| described_class.with_compiler(input:, filename:, options:, &b) }
        .to(yield_with_args(described_class))
    end

    context 'with Options.verbose = true' do
      let(:options) { build(:options, verbose: true) }

      it 'yields a Compiler with #benchmark set' do
        described_class.with_compiler(input:, filename:, options:) do |compiler|
          expect(compiler.benchmark).not_to(be_nil)
        end
      end
    end
  end

  describe '#outputting!' do
    it 'yields control' do
      expect { |b| compiler.outputting!(&b) }
        .to(yield_control)
    end
  end

  describe '#parse_template' do
    let(:template) { compiler.parse_template }
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
      let(:input) { 'foo,[[align=right/format=bold]]bar,baz' }

      it 'creates cells with the modifiers' do
        expect(template.rows[0].cells[1].modifier).to(be_aligned('right'))
        expect(template.rows[0].cells[1].modifier).to(be_formatted('bold'))
      end
    end

    context 'with cell modifiers with multiple values' do
      let(:input) { 'foo,[[align=right/format=bold/format=italic]]bar,baz' }

      it 'creates cells with the modifiers' do
        expect(template.rows[0].cells[1].modifier).to(be_formatted('bold'))
        expect(template.rows[0].cells[1].modifier).to(be_formatted('italic'))
      end
    end

    context 'with row modifiers' do
      let(:input) { '![[align=center/format=bold]]foo,bar,baz' }

      it 'creates rows with the modifiers' do
        expect(template.rows[0].modifier).to(be_aligned('center'))
        expect(template.rows[0].modifier).to(be_formatted('bold'))
      end
    end
  end

  describe '#parse_code_section!' do
    subject { compiler.parse_code_section! }

    context 'with no code section' do
      let(:input) { 'foo,bar,baz' }

      it { is_expected.not_to(be_nil) }

      it 'has empty variables' do
        expect(subject.variables).to(be_empty)
      end

      it 'has empty functions' do
        expect(subject.functions).to(be_empty)
      end
    end

    context 'with comments' do
      let(:input) do
        <<~INPUT
          # this is a comment
          ---
          foo,bar,bar
        INPUT
      end

      it 'has empty variables' do
        expect(subject.variables).to(be_empty)
      end

      it 'has empty functions' do
        expect(subject.functions).to(be_empty)
      end
    end

    context 'with variable definitions' do
      let(:input) do
        <<~INPUT
          foo := 1
          ---
          foo,bar,baz
        INPUT
      end

      it 'sets a variable' do
        expect(subject.variables).to(eq({ foo: build(:number_one) }))
      end
    end

    context 'with function definitions' do
      context 'a function with no arguments' do
        let(:input) do
          <<~INPUT
            def foo() indirect("c1")
            ---
            foo,bar,baz
          INPUT
        end

        it 'sets the function on functions' do
          expect(subject.functions).to(
            eq(
              {
                foo: build(
                  :fn,
                  name: :foo,
                  arguments: [],
                  body: build(:fn_call, name: :indirect, arguments: [build(:string, s: 'c1')])
                )
              }
            )
          )
        end
      end

      context 'a function with arguments' do
        let(:input) do
          <<~INPUT
            def foo(a, b) add($$a, $$b)
            ---
            foo,bar,baz
          INPUT
        end

        it 'sets the function on functions' do
          expect(subject.functions).to(
            eq(
              {
                foo: build(
                  :fn,
                  name: :foo,
                  arguments: %i[a b],
                  body: build(:fn_call, name: :add, arguments: [build(:variable, id: 'a'), build(:variable, id: 'b')])
                )
              }
            )
          )
        end
      end
    end

    context 'with key_values' do
      let(:input) do
        <<~INPUT
          foo := 1
          ---
          foo,bar,baz
        INPUT
      end
      let(:key_values) { { fooz: 'bar' } }

      it 'sets the variable, inferring the type as a string' do
        expect(subject.variables).to(eq({ foo: build(:number_one), fooz: build(:string, s: 'bar') }))
      end

      context 'with the same name as a variable' do
        let(:key_values) { { foo: 'bar' } }

        it 'they should overwrite any defined variables' do
          expect(subject.variables).to(eq({ foo: build(:string, s: 'bar') }))
        end
      end
    end
  end

  describe '#parse_csv_section!' do
    let(:input) { "foo,bar,baz\nfoo1,bar1,baz1\nfoo2,bar2,baz2\n" }

    subject { compiler.parse_csv_section! }

    it 'parses the CSV rows' do
      expect(subject.length).to(eq(3))
    end

    context 'with modifiers' do
      let(:input) { "[[align=right]]foo,bar,baz\n[[note='test']]foo1,bar1,baz1\n[[format=bold]]foo2,bar2,baz2\n" }

      it 'parses the CSV rows' do
        expect(subject.length).to(eq(3))
      end
    end
  end

  describe '#parse_row' do
    let(:values) { ['foo', '=ADD(1 ,2)', '=$$var'] }

    subject(:row) { compiler.parse_row(values) }

    it { is_expected.to(be_a(::CSVPlusPlus::Row)) }

    it 'sets rows.index' do
      expect(row.index).to(eq(0))
    end

    it 'sets cell.index' do
      expect(row.cells[0].index).to(eq(0))
      expect(row.cells[1].index).to(eq(1))
      expect(row.cells[2].index).to(eq(2))
    end

    it 'sets cell.row_index' do
      expect(row.cells[0].row_index).to(eq(row.index))
      expect(row.cells[1].row_index).to(eq(row.index))
      expect(row.cells[2].row_index).to(eq(row.index))
    end

    it 'sets cell.ast' do
      expect(row.cells[0].ast).to(be_nil)
      expect(row.cells[1].ast).to(eq(build(:fn_call_add)))
      expect(row.cells[2].ast).to(eq(build(:variable, id: :var)))
    end

    context 'with a cell modifier' do
      let(:values) { ['[[format=bold]]foo', 'bar', 'baz'] }

      it 'does not set the modifier on the row' do
        expect(row.modifier).not_to(be_formatted('bold'))
      end

      it 'sets bold only on one cell' do
        expect(row.cells[0].modifier).to(be_formatted('bold'))
        expect(row.cells[1].modifier).not_to(be_formatted('bold'))
        expect(row.cells[2].modifier).not_to(be_formatted('bold'))
      end
    end

    describe 'a row modifier provides defaults for the row' do
      let(:values) { ['![[format=bold]]foo', 'bar', 'baz'] }

      it 'sets bold on the row' do
        expect(row.modifier).to(be_formatted('bold'))
      end

      it 'sets bold on each cell' do
        expect(row.cells[0].modifier).to(be_formatted('bold'))
        expect(row.cells[1].modifier).to(be_formatted('bold'))
        expect(row.cells[2].modifier).to(be_formatted('bold'))
      end
    end
  end

  describe '#resolve_all_cells!' do
    let(:template) { build(:template, rows:, scope:) }
    let(:scope) { build(:scope, code_section:) }
    let(:code_section) { build(:code_section, variables:) }
    let(:cells_row0) do
      [
        build(:cell, row_index: 0, index: 0, value: '=$$foo', ast: build(:variable, id: :foo)),
        build(:cell, row_index: 0, index: 1, value: '=foo'),
        build(:cell, row_index: 0, index: 2, value: 'foo'),
        build(:cell, row_index: 0, index: 3, value: '=$$rownum', ast: build(:variable, id: :rownum))
      ]
    end
    let(:rows) { [build(:row, index: 0, cells: cells_row0)] }
    let(:variables) { { foo: build(:number_one) } }

    before { compiler.resolve_all_cells!(template) }

    it 'resolves the first one and leaves the others alone' do
      expect(template.rows[0].cells[0].to_csv).to(eq('=1'))
      expect(template.rows[0].cells[1].to_csv).to(eq('=foo'))
      expect(template.rows[0].cells[2].to_csv).to(eq('foo'))
    end

    it 'resolves runtime variables' do
      expect(template.rows[0].cells[3].to_csv).to(eq('=1'))
    end
  end

  describe '#to_s' do
    subject { compiler.to_s }

    it { is_expected.to(match(/Compiler\(options: Options\(.*\), runtime: Runtime\(.*\), scope: Scope\(.*\)/)) }
  end
end
