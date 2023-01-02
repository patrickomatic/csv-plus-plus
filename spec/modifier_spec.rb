require 'modifier'

describe CSVPlusPlus::Modifier do
  let (:modifier) { CSVPlusPlus::Modifier.new }

  describe "borders" do
    context "with a single values" do
      before do
        modifier.border = 'top'
        modifier.border = 'left'
      end

      it "sets top & left borders" do
        expect(modifier).to be_border_top
        expect(modifier).to be_border_left
      end
    end

    context "with 'all'" do
      before { modifier.border = 'all' }

      it "sets all borders" do
        expect(modifier).to be_border_top
        expect(modifier).to be_border_left
        expect(modifier).to be_border_right
        expect(modifier).to be_border_bottom
      end
    end
  end

  describe "formats" do
    context "with a single values" do
      before do
        modifier.format = 'bold'
        modifier.format = 'strikethrough'
      end

      it "sets formats" do
        expect(modifier).to be_bold
        expect(modifier).to be_strikethrough
      end
    end
  end

  describe "expand" do
    let(:amount) { nil }
    before { modifier.expand = expand }
    subject(:expand) { CSVPlusPlus::Modifier::Expand.new amount }

    it { is_expected.to be_infinite }

    context "with an amount" do
      let(:amount) { 2 }

      it { is_expected.not_to be_infinite }
    end
  end
end
