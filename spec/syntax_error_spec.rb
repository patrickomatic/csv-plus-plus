require 'syntax_error'

describe GSPush::SyntaxError do
  describe "#to_s" do
    let(:syntax_error) { GSPush::SyntaxError.new('test', 'bad input') } 
    subject { syntax_error.to_s }
        
    it { should eq('gspush: test: bad input') }

    context "with a line number" do
      let(:syntax_error) { GSPush::SyntaxError.new('test', 'bad input', line_number: 10) } 
      it { should eq('gspush: test 10: bad input') }
    end

    context "with a cell and row number" do
      let(:syntax_error) { GSPush::SyntaxError.new('test', 'bad input', row_number: 0, cell_number: 5) } 
      it { should eq('gspush: test 0:5: bad input') }
    end
  end
end
