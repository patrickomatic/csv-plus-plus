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
        expect(cell_modifier.align).to eq(['left'])
      end
    end

    context "multiple modifiers" do
      let(:value) { "[[align=left/format=bold/format=underline]]=A + B" }

      it { is_expected.to eq "=A + B" }

      it "updates cell_modifier" do
        expect(cell_modifier).to be_bold
        expect(cell_modifier).to be_underline
        expect(cell_modifier.align).to eq(['left'])
      end
    end

    context "row-based modifiers" do
      let(:value) { "![[align=center / format=bold]]Stocks" }

      it { is_expected.to eq "Stocks" }

      it "updates row_modifier" do
        expect(row_modifier).to be_bold
        expect(row_modifier.align).to eq(['center'])
      end
    end
  end
end
