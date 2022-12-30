require 'modifier.tab'

describe CSVPlusPlus::ModifierParser do
  describe "#parse" do
    subject { CSVPlusPlus::ModifierParser.new.parse(modifier) }

    context "without a modifier" do
      let(:modifier) { "foo" }
      it { should be_nil }
    end

    context "one modifier" do
      let(:modifier) { "[[align=left]]" }

      it "creates a modifier with align=left" do
        expect(subject.align).to eq(['left'])
      end
    end

    context "multiple modifiers" do
      let(:modifier) { "[[align=left/format=bold/format=underline]]" }

      it { should be_bold }
      it { should be_underline }

      it "creates a modifier" do
        expect(subject.align).to eq(['left'])
      end
    end

    context "row-based modifier" do
      let(:modifier) { "![[align=center / format=bold]]" }

      it { should be_row_level }
    end
  end
end
