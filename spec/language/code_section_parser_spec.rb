# frozen_string_literal: true

require 'code_section.tab'

describe ::CSVPlusPlus::Language::CodeSectionParser do
  describe '#parse' do
    let(:runtime) { build(:runtime) }
    let(:sections) { described_class.new.parse(::StringIO.new(input), runtime) }
    let(:code_section) { sections[0] }
    let(:csv_section) { sections[1] }

    describe 'CodeSection#variables' do
      subject { code_section.variables }

      context 'with comments' do
        let(:input) do
          <<~INPUT
            # this is a comment
            ---
            foo,bar,baz
          INPUT
        end

        it { is_expected.to(eq({})) }

        it 'returns the csv section' do
          expect(csv_section).to(eq('foo,bar,baz'))
        end
      end

      context 'with a bunch of spacing' do
        let(:input) do
          <<~INPUT


            ---
            foo,bar,baz
          INPUT
        end

        it { is_expected.to(eq({})) }

        it 'returns the csv section' do
          expect(csv_section).to(eq('foo,bar,baz'))
        end
      end

      context 'with a syntax error' do
        let(:input) do
          <<~INPUT
            foo cks,C<>c.
            .ccj
            kj:= 1
            ---
            =$$foo,bar,baz
          INPUT
        end

        it 'raises an error' do
          expect { subject }
            .to(raise_error(::CSVPlusPlus::Language::SyntaxError))
        end
      end

      context 'with a simple variable definition' do
        let(:input) do
          <<~INPUT
            foo := 1
            ---
            =$$foo,bar,baz
          INPUT
        end

        it { is_expected.to(eq({ foo: build(:number_one) })) }

        it 'returns the csv section' do
          expect(csv_section).to(eq('=$$foo,bar,baz'))
        end
      end

      context 'with cell references' do
        let(:input) do
          <<~INPUT
            foo := A1
            bar := A1:Z1
            baz := OtherSheet!A1:Z1
            c := A
            ---
            =SUM($$foo),bar,baz
          INPUT
        end

        it 'parses a cell reference' do
          expect(subject[:foo]).to(eq(build(:cell_reference, ref: 'A1')))
        end

        it 'parses a column reference' do
          expect(subject[:c]).to(eq(build(:cell_reference, ref: 'A')))
        end

        it 'parses a range reference' do
          expect(subject[:bar]).to(eq(build(:cell_reference, ref: 'A1:Z1')))
        end

        it 'parses a sheet reference' do
          expect(subject[:baz]).to(eq(build(:cell_reference, ref: 'OtherSheet!A1:Z1')))
        end
      end

      context 'with a variable definition with function calls' do
        let(:input) do
          <<~INPUT
            foo := ADD(MULTIPLY(C1, 8), $$var)
            ---
            =$$foo,bar,baz
          INPUT
        end

        it do
          is_expected.to(
            eq(
              {
                foo: build(
                  :fn_call,
                  name: 'ADD',
                  arguments: [
                    build(:fn_call, name: 'MULTIPLY', arguments: [build(:cell_reference), build(:number, n: 8)]),
                    build(:variable, id: 'var')
                  ]
                )
              }
            )
          )
        end
      end

      context 'with a variable referencing other variables' do
        let(:input) do
          <<~INPUT
            foo := 1
            bar := ADD($$foo, 2)
            ---
            =$$foo,=$$bar,baz
          INPUT
        end

        it do
          is_expected.to(
            eq(
              {
                foo: build(:number_one),
                bar: build(:fn_call, name: 'ADD', arguments: [build(:variable, id: 'foo'), build(:number_two)])
              }
            )
          )
        end

        it 'returns the csv section' do
          expect(csv_section).to(eq('=$$foo,=$$bar,baz'))
        end
      end

      context 'with an function with a single arg' do
        let(:input) do
          <<~INPUT
            foo := BAR(1)
            ---
            =$$foo,bar,baz
          INPUT
        end

        it do
          is_expected.to(eq({ foo: build(:fn_call, name: :bar, arguments: [build(:number_one)]) }))
        end
      end
    end

    describe 'CodeSection#functions' do
      subject { code_section.functions }

      context 'with a single function that takes no args' do
        let(:input) do
          <<~INPUT
            def foo() INDIRECT("BAR")
            ---
            =$$foo(A1, B1),bar,baz
          INPUT
        end

        it { is_expected.to(eq({ foo: build(:fn_foo) })) }
      end

      context 'with a single function that takes multiple args' do
        let(:fn_add) { build(:fn_add) }
        let(:input) do
          <<~INPUT
            def foo(a, b) ADD($$a, $$b)
            ---
            =$$foo(A1, B1),bar,baz
          INPUT
        end

        it { is_expected.to(eq({ foo: build(:fn, name: :foo, arguments: fn_add.arguments, body: fn_add.body) })) }
      end
    end
  end
end
