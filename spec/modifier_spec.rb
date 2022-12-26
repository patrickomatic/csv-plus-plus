require 'modifier'

describe GSPush::Modifier do
  describe ".get_modifier_from_value" do
    subject(:modifier) { GSPush::Modifier.get_modifier_from_value(cell_value) }
    let(:cell_value) { "[[format=bold]]bar" }

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
          expect { subject }.to raise_error(GSPush::Modifier::SyntaxError)
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
          expect { subject }.to raise_error(GSPush::Modifier::SyntaxError)
        end
      end
    end

    describe "expand=" do
      let(:cell_value) { "![[expand=2:3]]foo" }
      it "sets an ExpandRange" do
        expect(subject.expand.start_row).to eq(2)
        expect(subject.expand.end_row).to eq(3)
      end

      context "without an end" do
        let(:cell_value) { "![[expand=2:]]foo" }

        it "sets an ExpandRange" do
          expect(subject.expand.start_row).to eq(2)
          expect(subject.expand.end_row).to be_nil
        end
      end
    end

    context "with row-level modifiers" do
      let(:cell_value) { "![[format=bold]]bar" }
      it { should be_bold }
      it { should be_row_level }
    end

    context "with unsupported modifier" do
      let(:cell_value) { "[[foo=bar]]bar" }
      it "raises a syntax error" do
        expect { subject }.to raise_error(GSPush::Modifier::SyntaxError)
      end
    end
  end 
end
