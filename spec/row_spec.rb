require 'row'

describe CSVPlusPlus::Row do
  describe "#parse" do
    let(:values) { %w[foo bar baz] }
    subject { CSVPlusPlus::Row.parse_row(values, 0) }

    it { is_expected.to be_a CSVPlusPlus::Row }

    describe "a row modifier provides defaults for the row" do
      let(:values) { ['![[format=bold]]foo', 'bar', 'baz'] }
       
      it "sets bold on each cell" do
        expect(subject.cells[0].modifier).to be_bold 
        expect(subject.cells[1].modifier).to be_bold 
        expect(subject.cells[2].modifier).to be_bold 
      end
    end
  end
end
