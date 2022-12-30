require 'modifier'

describe CSVPlusPlus::Modifier do
  let (:modifier) { CSVPlusPlus::Modifier.new }

  describe "#borders=" do
    context "with a single values" do
      before do
        modifier.borders = 'top'
        modifier.borders = 'left'
      end

      it "sets borders" do
        expect(modifier.borders).to eq(%w[top left])
      end
    end
  end

  describe "#formats=" do
    context "with a single values" do
      before do
        modifier.formats = 'bold'
        modifier.formats = 'strikethrough'
      end

      it "sets formats" do
        expect(modifier.formats).to eq(%w[bold strikethrough])
      end
    end
  end
end
