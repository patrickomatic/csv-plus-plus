require 'code_section.tab'

describe CSVPlusPlus::Language::CodeSectionParser do
  let(:ec) { build(:execution_context, input:) }

  describe "#parse" do
    subject { described_class.new.parse(ec).variables }

    describe "#variables" do
      context "with comments" do
        let(:input) do
  "
  # this is a comment
  ---
  foo,bar,baz
  " 
        end

        it { is_expected.to eq({}) }
      end

      context "with a bunch of spacing" do
        let(:input) do
  "


  ---
  foo,bar,baz
  " 
        end

        it { is_expected.to eq({}) }
      end

      context "with a simple variable definition" do
        let(:input) do
  "
  foo := 1
  ---
  =$$foo,bar,baz
  " 
        end

        it { is_expected.to eq({ foo: build(:number_one) }) }
      end

      context "with a variable definition with function calls" do
        let(:input) do
  "
  foo := ADD(MULTIPLY(C1, 8), $$var)
  ---
  =$$foo,bar,baz
  " 
        end

        it do
          is_expected.to eq({
            foo: build(:fn_call,
                       name: 'ADD',
                       arguments: [
                         build(:fn_call,
                               name: 'MULTIPLY',
                               arguments: [
                                 build(:cell_reference),
                                 build(:number, n: 8)]),
                         build(:variable, id: 'var')])
          })
        end
      end

      context "with a variable referencing other variables" do
        let(:input) do
  "
  foo := 1
  bar := ADD($$foo, 2)
  ---
  =$$foo,=$$bar,baz
  " 
        end

        it do
          is_expected.to eq({ 
            foo: build(:number_one), 
            bar: build(:fn_call, 
                       name: "ADD",
                       arguments: [
                         build(:variable, id: "foo"),
                         build(:number_two)]),
          })
        end
      end

      context "with an function with a single arg" do
        let(:input) do
  "
  foo := BAR(1)
  ---
  =$$foo,bar,baz
  " 
        end

        it do
          is_expected.to eq({ 
            foo: build(:fn_call, name: :bar, arguments: [build(:number_one)]) 
          })
        end
      end
    end

    describe "#functions" do
      # XXX 
    end
  end
end
