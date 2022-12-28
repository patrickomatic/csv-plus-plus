require 'tempfile'
require 'code_section'

describe GSPush::CodeSection do
  describe "::parse!" do
    subject { GSPush::CodeSection.parse!(input) }

    let(:input) do
      Tempfile.new.tap do |f| 
        f.write(file_contents) 
        f.rewind
      end
    end
    after(:each) { input.close! }

    context "with no code section" do
      let(:file_contents) { 'foo,bar,baz' }

      it { should_not be_nil }
      it "has empty variables" do
        expect(subject.variables).to be_empty
      end
    end

    context "with comments" do
      let(:file_contents) { "# this is a comment\n" }

      it { should_not be_nil }
      it "has empty variables" do
        expect(subject.variables).to be_empty
      end
    end

    context "with variable definitions" do
      let(:file_contents) { "foo := 1\n" }

      it { should_not be_nil }
      it "sets a variable" do
        expect(subject.variables).to eq({ "foo" => "1" })
      end
    end
  end
end
