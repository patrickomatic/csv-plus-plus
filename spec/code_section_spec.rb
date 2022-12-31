require 'tempfile'
require 'code_section'

describe CSVPlusPlus::CodeSection do
  describe "::parse!" do
    subject { CSVPlusPlus::CodeSection.parse!(input) }
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
      it "leaves the CSV in the tempfile" do
        expect(input.read).to eq("foo,bar,baz")
      end
    end

    context "with comments" do
      let(:file_contents) { "# this is a comment\n---\nfoo,bar,bar" }

      it { should_not be_nil }
      it "has empty variables" do
        expect(subject.variables).to be_empty
      end
      it "leaves the CSV in the tempfile" do
        subject
        expect(input.read).to eq("foo,bar,bar")
      end
    end

    context "with variable definitions" do
      let(:file_contents) { "foo := 1\n---\nfoo,bar,baz" }

      it { should_not be_nil }
      it "sets a variable" do
        expect(subject.variables).to eq({ "foo" => [:number, 1] })
      end
      it "leaves the CSV in the tempfile" do
        subject
        expect(input.read).to eq("foo,bar,baz")
      end
    end
  end
end
