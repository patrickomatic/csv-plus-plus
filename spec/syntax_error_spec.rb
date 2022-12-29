require 'syntax_error'

describe CSVPlusPlus::SyntaxError do
  describe "#to_s" do
    let(:syntax_error) { CSVPlusPlus::SyntaxError.new('test', 'bad input') } 
    subject { syntax_error.to_s }
        
    it { should eq('csv++: test: bad input') }

    context "with a line number" do
      let(:syntax_error) { CSVPlusPlus::SyntaxError.new('test', 'bad input', line_number: 10) } 
      it { should eq('csv++: test 10: bad input') }
    end

    context "with a cell and row number" do
      let(:syntax_error) { CSVPlusPlus::SyntaxError.new('test', 'bad input', row_number: 0, cell_number: 5) } 
      it { should eq('csv++: test 0:5: bad input') }
    end
  end
end
