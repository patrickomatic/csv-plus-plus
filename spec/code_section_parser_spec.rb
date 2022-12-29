require 'code_section_parser.tab'

describe GSPush::CodeSectionParser do
  let(:code_section) { "" }
  subject { GSPush::CodeSectionParser.new.parse(code_section) }

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

  describe "extra spacing" do
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
foo,bar,baz
" 
    end

    it { should eq({"foo" => [:literal, "1"]}) }
  end

  describe "a variable definition with function calls" do
    let(:code_section) do
"
foo := ADD(MULTIPLY(C1, 8), 42)
---
foo,bar,baz
" 
    end

    it { should eq({"foo" => ["ADD", [["MULTIPLY", [[:literal, "C1"], [:literal, "8"]]], [:literal, "42"]]]}) }
  end
end
