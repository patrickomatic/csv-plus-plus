require 'modifier.tab'
require 'modifier'

describe CSVPlusPlus::ModifierParser do
  describe "#parse" do
    let(:row_modifier) { CSVPlusPlus::Modifier.new }
    let(:cell_modifier) { CSVPlusPlus::Modifier.new }

    before(:each) { subject }
    subject { CSVPlusPlus::ModifierParser.new.parse(value, cell_modifier:, row_modifier:) }

    context "without a modifier" do
      let(:value) { "foo" }

      it { is_expected.to eq "foo" }
    end

    context "one modifier" do
      let(:value) { "[[align=left]]foo" }

      it { is_expected.to eq "foo" }

      it "updates the cell_modifier with align=left" do
        expect(cell_modifier).to be_left_align
      end
    end

    context "multiple modifiers" do
      let(:value) { "[[align=left/format=bold/format=underline]]=A + B" }

      it { is_expected.to eq "=A + B" }

      it "updates cell_modifier" do
        expect(cell_modifier).to be_bold
        expect(cell_modifier).to be_underline
        expect(cell_modifier).to be_left_align
      end
    end

    context "row-based modifiers" do
      let(:value) { "![[align=center / format=bold]]Stocks" }

      it { is_expected.to eq "Stocks" }

      it "updates row_modifier" do
        expect(row_modifier).to be_bold
        expect(row_modifier).to be_center_align
      end
    end

    context "a note" do
      let(:value) { "[[note='this is a note']]=A + B" }

      it { is_expected.to eq "=A + B" }

      it "sets the note" do
        expect(cell_modifier.note).to eq('this is a note')
      end
    end

    context "a color" do
      let(:value) { "[[color=#FF00FF]]=ADD(1, 2)" }

      it { is_expected.to eq "=ADD(1, 2)" }

      it "sets the color" do
        expect(cell_modifier.color.red).to eq 1
        expect(cell_modifier.color.green).to eq 0
        expect(cell_modifier.color.blue).to eq 1
      end
    end
  end
end
