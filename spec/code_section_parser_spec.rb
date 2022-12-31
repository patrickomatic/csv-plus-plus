require 'code_section.tab'

describe CSVPlusPlus::CodeSectionParser do
  describe "#parse" do
    subject { CSVPlusPlus::CodeSectionParser.new.parse(code_section).variables }

    describe "comments" do
      let(:code_section) do
"
# this is a comment
---
foo,bar,baz
" 
      end

      it { should eq({}) }
    end

    describe "a bunch of spacing" do
      let(:code_section) do
"


---
foo,bar,baz
" 
      end

      it { should eq({}) }
    end

    describe "a simple variable definition" do
      let(:code_section) do
"
foo := 1
---
=$$foo,bar,baz
" 
      end

      it { should eq({"foo" => [:number, 1]}) }
    end

    describe "a variable definition with function calls" do
      let(:code_section) do
"
foo := ADD(MULTIPLY(C1, 8), $$var)
---
=$$foo,bar,baz
" 
      end

      it do
        should eq({
          "foo" => [[:fn, "ADD"], 
                    [
                      [[:fn, "MULTIPLY"], 
                       [[:id, "C1"], [:number, 8]]],
                      [:var, "var"]]]
        }) 
      end
    end

    describe "a variable referencing other variables" do
      let(:code_section) do
"
foo := 1
bar := ADD($$foo, 2)
---
=$$foo,=$$bar,baz
" 
      end

      it do
        should eq({ 
          "foo" => [:number, 1], 
          "bar" => [[:fn, "ADD"], [[:var, "foo"], [:number, 2]]] 
        })
      end
    end

    describe "an infix function" do
      let(:code_section) do
"
foo := ADD(1, 2)
---
=$$foo,bar,baz
" 
      end

      it { should eq({ "foo" => [[:fn, "ADD"], [[:number, 1], [:number, 2]]] }) }
    end

    describe "an function with a single arg" do
      let(:code_section) do
"
foo := BAR(1)
---
=$$foo,bar,baz
" 
      end

      it { should eq({ "foo" => [[:fn, "BAR"], [[:number, 1]]] }) }
    end

    xdescribe "parenthesis to group operations" do
      let(:code_section) do
"
foo := (1 + 2) * 8
---
=$$foo,bar,baz
" 
      end

      it do
        should eq({ 
          "foo" => [[:fn, "MULTIPLY"], [[[:fn, "ADD"], [[:number, 1], [:number, 2]]]], [:number, 8]],
        })
      end
    end
  end
end
