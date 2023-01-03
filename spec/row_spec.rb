require 'row'

describe CSVPlusPlus::Row do
  describe "#parse" do
    let(:values) { %w[foo bar baz] }

    subject(:row) { CSVPlusPlus::Row.parse_row(values, 0) }

    it { is_expected.to be_a CSVPlusPlus::Row }

    it "sets rows.index" do
      expect(row.index).to eq 0
    end

    it "sets cell.index" do
      expect(row.cells[0].index).to eq 0
      expect(row.cells[1].index).to eq 1
      expect(row.cells[2].index).to eq 2
    end

    it "sets cell.row_index" do
      expect(row.cells[0].row_index).to eq 0
      expect(row.cells[1].row_index).to eq 0
      expect(row.cells[2].row_index).to eq 0
    end

    context "with a cell modifier" do
      let(:values) { ['[[format=bold]]foo', 'bar', 'baz'] }

      it "does not set the modifier on the row" do
        expect(row.modifier).not_to be_bold
      end

      it "sets bold only on one cell" do
        expect(row.cells[0].modifier).to be_bold
        expect(row.cells[1].modifier).not_to be_bold
        expect(row.cells[2].modifier).not_to be_bold
      end
    end

    describe "a row modifier provides defaults for the row" do
      let(:values) { ['![[format=bold]]foo', 'bar', 'baz'] }
       
      it "sets bold on the row" do
        expect(row.modifier).to be_bold 
      end
      
      it "sets bold on each cell" do
        expect(row.cells[0].modifier).to be_bold 
        expect(row.cells[1].modifier).to be_bold 
        expect(row.cells[2].modifier).to be_bold 
      end
    end
  end

  describe "#expand_amount" do
    let(:expand_amount) { 2 }
    let(:row_index) { 0 }
    let(:cells) { [] }
    let(:modifier) { CSVPlusPlus::Modifier.new }
    let(:row) { CSVPlusPlus::Row.new(row_index, cells, modifier) }

    before { modifier.expand = CSVPlusPlus::Modifier::Expand.new expand_amount }

    subject { row.expand_amount }

    it { is_expected.to eq 2 }

    context "when no amount is set" do
      let(:expand_amount) { nil }

      it { is_expected.to eq 1000 }

      context "and the row is offset" do
        let(:row_index) { 2 }

        it { is_expected.to eq 998 }
      end
    end
  end

  describe "#index=" do
    let(:expand_amount) { 2 }
    let(:row_index) { 0 }
    let(:cells) do
      [
        CSVPlusPlus::Cell.new(row_index, 0, 'foo', CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(row_index, 1, 'bar', CSVPlusPlus::Modifier.new),
        CSVPlusPlus::Cell.new(row_index, 2, 'baz', CSVPlusPlus::Modifier.new),
      ] 
    end
    let(:modifier) { CSVPlusPlus::Modifier.new }
    let(:row) { CSVPlusPlus::Row.new(row_index, cells, modifier) }

    before { row.index = 10 }

    it "sets the value" do
      expect(row.index).to eq 10
    end

    it "propagates the change to each cell.row_index" do
      expect(row.cells[0].row_index).to eq 10
      expect(row.cells[1].row_index).to eq 10
      expect(row.cells[2].row_index).to eq 10
    end
  end
end
