require 'modifier'

describe CSVPlusPlus::Modifier do
  describe ".get_modifier_from_value" do
    subject(:modifier) do
      CSVPlusPlus::Modifier.get_modifier_from_value(cell_value, row_number, cell_number) 
    end
    let(:cell_value) { "[[format=bold]]bar" }
    let(:cell_number) { 1 }
    let(:row_number) { 1 }

    describe "format=" do
      it { should be_bold }

      context "with multiple formats" do
        let(:cell_value) { "[[format=bold italic]]bar" }
        it { should be_italic }
        it { should be_bold }
      end

      context "with invalid formats" do
        let(:cell_value) { "[[format=foo]]bar" }
        it "raises a syntax error" do
          expect { subject }.to raise_error(CSVPlusPlus::SyntaxError)
        end
      end
    end

    describe "align=" do
      let(:cell_value) { "[[align=center]]foo" }
      it "handles a single align" do
        expect(subject.align).to eq("center")
      end

      context "with invalid align" do
        let(:cell_value) { "[[align=foo]]foo" }
        it "raises a syntax error" do
          expect { subject }.to raise_error(CSVPlusPlus::SyntaxError)
        end
      end
    end

    describe "expand=" do
      let(:cell_value) { "![[expand=2]]foo" }
      it "sets an ExpandRange" do
        expect(subject.expand.repetitions).to eq(2)
        expect(subject.expand.infinite?).to be(false)
      end

      context "without repetitions" do
        let(:cell_value) { "![[expand]]foo" }
        it "should be infinite" do
          expect(subject.expand.infinite?).to be(true)
        end
      end
    end

    context "with row-level modifiers" do
      let(:cell_value) { "![[format=bold]]bar" }
      it { should be_bold }
      it { should be_row_level }
    end

    describe "incorrect modifiers" do
      context "with unsupported modifier" do
        let(:cell_value) { "[[foo=bar]]bar" }
        it "raises a syntax error" do
          expect { subject }.to raise_error(CSVPlusPlus::SyntaxError)
        end
      end

      context "with nil values" do
        let(:cell_value) { "bar,," }
        it { should be_nil }
      end
    end
  end 
end
